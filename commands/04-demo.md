# Manos a la obra · De la idea a testnet

Construye a mano un contrato Soroban de propinas y despliégalo en testnet.

## Sin IA

```bash
stellar contract init propinas
cd propinas
# escribe el contrato en Rust en contracts/hello-world/src/lib.rs
cargo test
stellar contract build
stellar contract deploy --wasm target/wasm32v1-none/release/hello_world.wasm --source-account alice --network testnet --alias propinas
```

## Fuente oficial

- https://developers.stellar.org/docs/build/smart-contracts/getting-started
- https://developers.stellar.org/docs/tools/cli
