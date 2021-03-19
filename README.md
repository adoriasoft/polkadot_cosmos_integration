# Substrate Node with Cosmos ABCI pallet

A new FRAME-based Substrate node with Cosmos ABCI pallet.

## Documentation

Documentation for this project is [here](https://github.com/adoriasoft/polkadot-cosmos-docs).

## Build in local environment

Install Rust:

```sh
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```sh
./scripts/init.sh
```

Build Wasm and native code:

```sh
cargo build --release
```

Build and start Cosmos node:
Go to the our fork of the [cosmos-sdk](https://github.com/adoriasoft/cosmos-sdk/tree/master), switch to the branch [feature/add_nameservice](https://github.com/adoriasoft/cosmos-sdk/tree/feature/add_nameservice), then follow the [instructions](https://github.com/adoriasoft/cosmos-sdk/tree/feature/add_nameservice/simapp).

#### Specify environment variables used by node

```sh
# Set ABCI backend url
export ABCI_SERVER_URL=tcp://localhost:26658
# Set path to Genesis file
export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
# Set whole Genesis state from file
export ABCI_GENESIS_STATE=$(cat $HOME/.nsd/config/genesis.json)
# Re-export whole Genesis state from file
export ABCI_GENESIS_STATE=$(cat $HOME/.nsd/config/genesis.json)
```

#### Build with selected consensus

````
Available consensuses
- `aura`
- `babe`

```sh
- cargo build --release --no-default-features --features <consensus_name>
````

## Build in dockerized environment

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain. Also, sometimes you have to share permission for the `.maintain` folder in Docker settings.

```sh
./scripts/docker_run.sh
```

Or:

```sh
docker-compose up -d
```

## Run node in development mode

Start a development chain with:

```sh
./target/release/node-template\
 --abci_genesis_state_path $HOME/.nsd/config/genesis.json\
 --abci_server_url tcp://localhost:26658\
 --abci_rpc_url 127.0.0.1:26657 --dev
```

Purge any existing development chain state:

```sh
./target/release/node-template purge-chain --dev
```

To get detailed info about options that available for node, run:

```sh
./target/release/node-template --help
```

To show detailed logs, run the the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

## Testing

#### Unit and integration testing with cargo

```sh
# Test all cargo packages
cargo test --all
# Test pallet-cosmos-abci package
cargo test -p pallet-cosmos-abci --test cosmos_abci_unit_test
cargo test -p pallet-cosmos-abci --test crypto_transform_unit_test
# Test pallet-abci package
cargo test -p pallet-abci --test abci_integration_test
cargo test -p pallet-abci --test abci_unit_test
```

#### Integration testing with bash

Follow the docs from the [directory](https://github.com/adoriasoft/polkadot_cosmos_integration/tree/master/scripts/integration_tests).


## How to use?

ABCI pallet allows launching applications based on Cosmos SDK (or any other application that uses ABCI) on top of Substrate consensus and network layers. When both Cosmos and Substrate nodes are running, users can interact with them using [Polkadot GUI](https://polkadot.js.org/apps/) and [Cosmos CLI](https://docs.cosmos.network/v0.39/interfaces/cli.html). 

In Polkadot GUI, users can send standard Substrate extrinsic including extrinsics to ABCI pallet to match Substrate and Cosmos accounts for validators elections and Cosmos transaction wrapped with Substrate extrinsics. In Cosmos CLI, users can make the majority of standards calls to Cosmos node (all available functions are described in the [full documentation](https://github.com/adoriasoft/polkadot-cosmos-docs). Some examples of commends for Nameservice app are described [here](https://github.com/adoriasoft/cosmos-sdk/blob/feature/add_nameservice/simapp/README.md).

### Perform calls to Cosmos RPC

- Simple request to Cosmos RPC:
  `{ "jsonrpc": "2.0", "method": <method_name>, "id": 0, "params": <method_params> }`
