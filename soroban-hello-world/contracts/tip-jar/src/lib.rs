#![no_std]
//! Tip jar: any account can tip a creator in a single SEP-41 token,
//! and each creator can withdraw the tips accumulated for them.

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, token, Address, Env,
};

const DAY_IN_LEDGERS: u32 = 17_280;
const TTL_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const TTL_EXTEND_TO: u32 = 90 * DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Token accepted by this tip jar (instance storage).
    Token,
    /// Withdrawable balance owed to a creator (persistent storage).
    Balance(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
    NothingToWithdraw = 2,
    SelfTip = 3,
    Overflow = 4,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tipped {
    #[topic]
    pub from: Address,
    #[topic]
    pub creator: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Withdrawn {
    #[topic]
    pub creator: Address,
    pub amount: i128,
}

#[contract]
pub struct TipJar;

#[contractimpl]
impl TipJar {
    /// Runs once at deploy time and fixes the token used for every tip.
    pub fn __constructor(env: Env, token: Address) {
        env.storage().instance().set(&DataKey::Token, &token);
    }

    /// Moves `amount` of the token from `from` into the contract and
    /// credits it to `creator`. Requires `from` to authorize.
    pub fn tip(env: Env, from: Address, creator: Address, amount: i128) -> Result<i128, Error> {
        from.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if from == creator {
            return Err(Error::SelfTip);
        }

        let key = DataKey::Balance(creator.clone());
        let new_balance = Self::balance(env.clone(), creator.clone())
            .checked_add(amount)
            .ok_or(Error::Overflow)?;

        token::TokenClient::new(&env, &Self::token(env.clone()))
            .transfer(&from, &env.current_contract_address(), &amount);

        env.storage().persistent().set(&key, &new_balance);
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
        Self::bump_instance(&env);

        Tipped { from, creator, amount }.publish(&env);
        Ok(new_balance)
    }

    /// Sends the creator's whole accumulated balance to the creator.
    /// Requires `creator` to authorize. Returns the amount withdrawn.
    pub fn withdraw(env: Env, creator: Address) -> Result<i128, Error> {
        creator.require_auth();

        let key = DataKey::Balance(creator.clone());
        let amount = Self::balance(env.clone(), creator.clone());
        if amount <= 0 {
            return Err(Error::NothingToWithdraw);
        }

        // Clear state before the external call (checks-effects-interactions).
        env.storage().persistent().remove(&key);
        Self::bump_instance(&env);

        token::TokenClient::new(&env, &Self::token(env.clone()))
            .transfer(&env.current_contract_address(), &creator, &amount);

        Withdrawn { creator, amount }.publish(&env);
        Ok(amount)
    }

    /// Tips currently withdrawable by `creator`.
    pub fn balance(env: Env, creator: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(creator))
            .unwrap_or(0)
    }

    /// Token accepted by this tip jar.
    pub fn token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Token).unwrap()
    }

    fn bump_instance(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
    }
}

mod test;
