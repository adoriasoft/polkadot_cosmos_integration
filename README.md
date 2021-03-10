# Substrate Node with Cosmos ABCI pallet

A new FRAME-based Substrate node with Cosmos ABCI pallet.

## Documentation

Documentation for this project is [here](https://github.com/adoriasoft/polkadot-cosmos-docs)

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
cargo build
```

Before running substrate node, you should build and start cosmos node. Go to the our fork of the [cosmos-sdk](https://github.com/adoriasoft/cosmos-sdk/tree/master), switch to the branch [feature/add_nameservice](https://github.com/adoriasoft/cosmos-sdk/tree/feature/add_nameservice), then just follow [instractions](https://github.com/adoriasoft/cosmos-sdk/tree/feature/add_nameservice/simapp).

#### Specify environment variables that using by node

```sh
# Set ABCI backend url
export ABCI_SERVER_URL=tcp://localhost:26658
# Using path to Genesis file
export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
# Using whole Genesis state from file
export ABCI_GENESIS_STATE=$(cat $HOME/.nsd/config/genesis.json)
# Re-export whole Genesis state from file
export ABCI_GENESIS_STATE=$(cat $HOME/.nsd/config/genesis.json)
```

#### Build with selected consensus

````
Available consensus
- `aura`
- `babe`

```sh
- cargo build --no-default-features --features <consensus_name>
````

## Dockerize environment

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain. Also, sometimes you have to share permission for the `.maintain` folder in Docker settings.

```sh
./scripts/docker_run.sh
```

Or:

```sh
docker-compose up -d
```

## Testing

#### Unit and integration Testing with cargo

```sh
# Testing of all cargo packages
cargo test --all
# Testing of pallet-cosmos-abci package
cargo test -p pallet-cosmos-abci --test cosmos_abci_unit_test
cargo test -p pallet-cosmos-abci --test crypto_transform_unit_test
# Testing of pallet-abci package
cargo test -p pallet-abci --test abci_integration_test
cargo test -p pallet-abci --test abci_unit_test
```

#### Integration Testing with bash

Follow the docs from `https://github.com/adoriasoft/polkadot_cosmos_integration/tree/master/scripts/integration_tests` directory.

### Node development chain

Start a development chain with:

```sh
./target/release/node-template
  --abci_genesis_state_path $HOME/.nsd/config/genesis.json
  --abci_server_url tcp://localhost:26658
  --abci_rpc_url 127.0.0.1:26657
  --dev
```

Purge any existing developer chain state:

```sh
./target/release/node-template purge-chain --dev
```

To get detailed info about options that available for node:

```sh
./target/release/node-template --help
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Perform calls to Cosmos RPC

- Simple request to Cosmos RPC:
  `{ "jsonrpc": "2.0", "method": <method_name>, "id": 0, "params": <method_params> }`
