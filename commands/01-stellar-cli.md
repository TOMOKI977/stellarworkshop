# Paso 1 · Stellar CLI: de cero a testnet

Instala la CLI `stellar` y lleva tu primer contrato Soroban de cero a testnet.

## Sin IA

```bash
# Instalar
winget install --id Stellar.StellarCLI      # Windows
brew install stellar-cli                     # macOS / Linux
rustup target add wasm32v1-none              # Rust para contratos

# Primer contrato
stellar keys generate alice --network testnet --fund
stellar contract init soroban-hello-world
cd soroban-hello-world
stellar contract build
stellar contract deploy --wasm target/wasm32v1-none/release/hello_world.wasm --source-account alice --network testnet --alias hello_world
stellar contract invoke --id hello_world --source-account alice --network testnet -- hello --to RPC
```

## Fuente oficial

- https://developers.stellar.org/docs/tools/cli/install-cli
- https://developers.stellar.org/docs/build/smart-contracts/getting-started
