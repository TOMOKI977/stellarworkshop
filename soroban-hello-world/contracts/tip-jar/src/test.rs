#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Events as _},
    token::{StellarAssetClient, TokenClient},
    Address, Env, Event, IntoVal, Symbol,
};

struct Setup<'a> {
    env: Env,
    jar: TipJarClient<'a>,
    token: TokenClient<'a>,
    fan: Address,
    creator: Address,
}

fn setup() -> Setup<'static> {
    let env = Env::default();
    env.mock_all_auths();

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer);
    let token = TokenClient::new(&env, &sac.address());

    let fan = Address::generate(&env);
    let creator = Address::generate(&env);
    StellarAssetClient::new(&env, &sac.address()).mint(&fan, &1_000);

    let jar_id = env.register(TipJar, (sac.address(),));
    let jar = TipJarClient::new(&env, &jar_id);

    Setup { env, jar, token, fan, creator }
}

#[test]
fn constructor_stores_token() {
    let s = setup();
    assert_eq!(s.jar.token(), s.token.address);
}

#[test]
fn tip_moves_tokens_and_credits_creator() {
    let s = setup();

    assert_eq!(s.jar.tip(&s.fan, &s.creator, &100), 100);

    assert_eq!(s.token.balance(&s.fan), 900);
    assert_eq!(s.token.balance(&s.jar.address), 100);
    assert_eq!(s.jar.balance(&s.creator), 100);
    // Tips stay in the jar until the creator withdraws.
    assert_eq!(s.token.balance(&s.creator), 0);
}

#[test]
fn tip_requires_tipper_auth() {
    let s = setup();
    s.jar.tip(&s.fan, &s.creator, &50);

    assert_eq!(
        s.env.auths()[0],
        (
            s.fan.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    s.jar.address.clone(),
                    Symbol::new(&s.env, "tip"),
                    (s.fan.clone(), s.creator.clone(), 50_i128).into_val(&s.env),
                )),
                sub_invocations: std::vec![AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        s.token.address.clone(),
                        Symbol::new(&s.env, "transfer"),
                        (s.fan.clone(), s.jar.address.clone(), 50_i128).into_val(&s.env),
                    )),
                    sub_invocations: std::vec![],
                }],
            }
        )
    );
}

#[test]
fn tip_emits_event() {
    let s = setup();
    s.jar.tip(&s.fan, &s.creator, &25);

    let expected = Tipped {
        from: s.fan.clone(),
        creator: s.creator.clone(),
        amount: 25,
    };
    let events = s.env.events().all().filter_by_contract(&s.jar.address);
    assert_eq!(events, std::vec![expected.to_xdr(&s.env, &s.jar.address)]);
}

#[test]
fn tips_from_many_accounts_accumulate() {
    let s = setup();
    let other_fan = Address::generate(&s.env);
    StellarAssetClient::new(&s.env, &s.token.address).mint(&other_fan, &500);

    s.jar.tip(&s.fan, &s.creator, &100);
    s.jar.tip(&other_fan, &s.creator, &250);

    assert_eq!(s.jar.balance(&s.creator), 350);
    assert_eq!(s.token.balance(&s.jar.address), 350);
}

#[test]
fn balances_are_isolated_per_creator() {
    let s = setup();
    let other_creator = Address::generate(&s.env);

    s.jar.tip(&s.fan, &s.creator, &100);
    s.jar.tip(&s.fan, &other_creator, &40);

    assert_eq!(s.jar.withdraw(&other_creator), 40);
    assert_eq!(s.jar.balance(&s.creator), 100);
    assert_eq!(s.jar.balance(&other_creator), 0);
}

#[test]
fn withdraw_pays_creator_and_resets_balance() {
    let s = setup();
    s.jar.tip(&s.fan, &s.creator, &300);

    assert_eq!(s.jar.withdraw(&s.creator), 300);

    assert_eq!(s.token.balance(&s.creator), 300);
    assert_eq!(s.token.balance(&s.jar.address), 0);
    assert_eq!(s.jar.balance(&s.creator), 0);
}

#[test]
fn withdraw_requires_creator_auth() {
    let s = setup();
    s.jar.tip(&s.fan, &s.creator, &10);
    s.jar.withdraw(&s.creator);

    let (addr, invocation) = s.env.auths()[0].clone();
    assert_eq!(addr, s.creator);
    assert_eq!(
        invocation.function,
        AuthorizedFunction::Contract((
            s.jar.address.clone(),
            Symbol::new(&s.env, "withdraw"),
            (s.creator.clone(),).into_val(&s.env),
        ))
    );
}

#[test]
fn withdraw_emits_event() {
    let s = setup();
    s.jar.tip(&s.fan, &s.creator, &70);
    s.jar.withdraw(&s.creator);

    let expected = Withdrawn {
        creator: s.creator.clone(),
        amount: 70,
    };
    let events = s.env.events().all().filter_by_contract(&s.jar.address);
    assert_eq!(events, std::vec![expected.to_xdr(&s.env, &s.jar.address)]);
}

#[test]
fn withdraw_without_tips_fails() {
    let s = setup();
    assert_eq!(
        s.jar.try_withdraw(&s.creator),
        Err(Ok(Error::NothingToWithdraw))
    );
}

#[test]
fn second_withdraw_fails() {
    let s = setup();
    s.jar.tip(&s.fan, &s.creator, &10);
    s.jar.withdraw(&s.creator);

    assert_eq!(
        s.jar.try_withdraw(&s.creator),
        Err(Ok(Error::NothingToWithdraw))
    );
}

#[test]
fn tip_rejects_zero_and_negative_amounts() {
    let s = setup();
    assert_eq!(
        s.jar.try_tip(&s.fan, &s.creator, &0),
        Err(Ok(Error::InvalidAmount))
    );
    assert_eq!(
        s.jar.try_tip(&s.fan, &s.creator, &-5),
        Err(Ok(Error::InvalidAmount))
    );
    assert_eq!(s.token.balance(&s.fan), 1_000);
}

#[test]
fn tip_rejects_self_tip() {
    let s = setup();
    assert_eq!(
        s.jar.try_tip(&s.fan, &s.fan, &10),
        Err(Ok(Error::SelfTip))
    );
}

#[test]
fn tip_above_tipper_balance_fails_without_crediting() {
    let s = setup();
    assert!(s.jar.try_tip(&s.fan, &s.creator, &5_000).is_err());
    assert_eq!(s.jar.balance(&s.creator), 0);
    assert_eq!(s.token.balance(&s.fan), 1_000);
}

#[test]
#[should_panic]
fn tip_without_auth_panics() {
    let s = setup();
    s.env.set_auths(&[]);
    s.jar.tip(&s.fan, &s.creator, &10);
}
