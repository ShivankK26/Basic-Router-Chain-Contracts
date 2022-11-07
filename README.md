# router-smart-contract-examples
Sample CosmWasm smart contracts for the Router chain.

## Prerequisites

Before starting, make sure you have [rustup](https://rustup.rs/) along with a
recent `rustc` and `cargo` version installed. Currently, we are testing on 1.58.1+.

And you need to have the `wasm32-unknown-unknown` target installed as well.

You can check that via:

```sh
rustc --version
cargo --version
rustup target list --installed
# if wasm32 is not listed above, run this
rustup target add wasm32-unknown-unknown
```

## running tests
```
cargo test --locked
```

## Compile
```
cargo build --locked
```

## Optimized builds

Please use the following commands to build the optimized builds:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="cw-router-chain-contract",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.5

```

or run the following script
```
sh scripts/build_optimize_wasm.sh
``` 

## Contract deployment script
Please use the following instruction to deploy your contract on the Router chain
This script will store the wasm build on the chain and instantiate the contract with provided init msg
```
sh scripts/deploy_contract.sh <contract-name> <init_msg>
```

Example
```
sh scripts/deploy_contract.sh hello-router {\"bridge_address\":\"0x93775D3937d5B80313942697d41AE44960f8ef3C\"}
```
