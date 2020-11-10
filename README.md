# Substrate Node with Cosmos ABCI pallet

A new FRAME-based Substrate node with Cosmos ABCI.

## Documentation

Documentation for this project is [here](https://github.com/adoriasoft/polkadot-cosmos-docs)

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build
```

## Run

Before running substrate node, you should build and start cosmos node. Go to the our fork of the [cosmos-sdk](https://github.com/adoriasoft/cosmos-sdk/tree/master), switch to the branch [feature/add_nameservice](https://github.com/adoriasoft/cosmos-sdk/tree/feature/add_nameservice), then just follow [instractions](https://github.com/adoriasoft/cosmos-sdk/tree/feature/add_nameservice/simapp).

To run node locally you will need to specify the environment variables for cosmos-abci pallet.

```bash
export ABCI_SERVER_URL=tcp://localhost:26658
export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
# or
export ABCI_GENESIS_STATE=$(cat $HOME/.nsd/config/genesis.json)
```

After any updating of the genesis.json file should specify following encironment variable

```bash
export ABCI_GENESIS_STATE=$(cat $HOME/.nsd/config/genesis.json)
```

### Tests

To run tests from local use commands:

```sh
# pallet-cosmos-abci
cargo test --test pallet_abci_test
# abci
cargo test --test abci_test
```

#### Bash integration tests

Follow to the scripts/integration_tests directory

### Single Node Development Chain

Purge any existing developer chain state:

```bash
./target/release/node-template purge-chain --dev
```

Start a development chain with:

```bash
./target/release/node-template --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`) by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

### CosmosRPC REST calls

- Request to CosmosRPC API:
  `{ "jsonrpc": "2.0", "method": <method_name>, "id": 0, "params": <method_params> }`
