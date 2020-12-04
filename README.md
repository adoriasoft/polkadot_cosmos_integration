# Substrate Node with Cosmos ABCI pallet

A new FRAME-based Substrate node with Cosmos ABCI.

## Documentation

Documentation for this project is [here](https://github.com/adoriasoft/polkadot-cosmos-docs)

## Build

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
cargo build
```

## Run

Before running substrate node, you should build and start cosmos node. Go to the our fork of the [cosmos-sdk](https://github.com/adoriasoft/cosmos-sdk/tree/master), switch to the branch [feature/add_nameservice](https://github.com/adoriasoft/cosmos-sdk/tree/feature/add_nameservice), then just follow [instractions](https://github.com/adoriasoft/cosmos-sdk/tree/feature/add_nameservice/simapp).

To run node locally you will need to specify the environment variables for cosmos-abci pallet.

```sh
export ABCI_SERVER_URL=tcp://localhost:26658
export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
# or
export ABCI_GENESIS_STATE=$(cat $HOME/.nsd/config/genesis.json)
```

After any updating of the genesis.json file should specify following encironment variable

```sh
export ABCI_GENESIS_STATE=$(cat $HOME/.nsd/config/genesis.json)
```

### Tests

#### Pallets tests

To run tests from local use commands:

```sh
# pallet-cosmos-abci
cargo test --test pallet_abci_test
# abci
cargo test --test abci_integration_test
cargo test --test abci_unit_test
```

#### sh integration tests

Follow to the scripts/integration_tests directory

### Single Node Development Chain

Purge any existing developer chain state:

```sh
./target/release/node-template purge-chain --dev
```

Start a development chain with:

```sh
./target/release/node-template --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain. Also, sometimes you have to share permission for the `.maintain` folder in Docker settings.

```sh
./scripts/docker_run.sh
```

Or:

```sh
docker-compose up -d
```

### CosmosRPC REST calls

- Request to CosmosRPC API:
  `{ "jsonrpc": "2.0", "method": <method_name>, "id": 0, "params": <method_params> }`
