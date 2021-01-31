#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

COSMOS_NODE_1_HOME=/tmp/cosmos_node_1
COSMOS_NODE_2_HOME=/tmp/cosmos_node_2

SUBSTRATE_NODE_1_HOME=/tmp/substrate_node_1
SUBSTRATE_NODE_2_HOME=/tmp/substrate_node_2

function clean_tmp() {
    rm -rf tmp
    mkdir tmp
    touch tmp/substrate_log.log
    touch tmp/cosmos_log.log
}

function clean_substrate() {
    echo "Clean Substrate"
    export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
    ./../../target/debug/node-template purge-chain --dev -y
}

function start_substrate() {
    echo "Run Substrate"
    export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
    ./../../target/debug/node-template --dev &> tmp/substrate_log.log &
    export SUBSTRATE_PID=$!
}

function clean_cosmos() {
    echo "Clean Cosmos"
    nsd unsafe-reset-all
}

function start_cosmos() {
    echo "Setup cosmos application"
    nsd start --with-tendermint=false --transport=grpc &> tmp/cosmos_log.log &
    export COSMOS_PID=$!
}

########
# separate substarte node 1
 
function start_substrate_1() {
    echo "Run Substrate node 1"
    export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
    export ABCI_SERVER_URL=tcp://localhost:26660
    export ABCI_RPC_SERVER_URL=127.0.0.1:26659
    ./../../target/debug/node-template --base-path=$SUBSTRATE_NODE_1_HOME --chain=local --alice &> tmp/substrate_node_1_log.log &
    export SUBSTRATE_NODE_1_PID=$!

    unset ABCI_SERVER_URL
    unset DEFAULT_ABCI_RPC_URL
    unset ABCI_GENESIS_STATE_PATH
}

function clean_substrate_1() {
    echo "Clean Substrate node 1"
    ./../../target/debug/node-template purge-chain --base-path=$SUBSTRATE_NODE_1_HOME --chain=local
}

##

function start_substrate_2() {
    echo "Run Substrate node 2"
    export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
    export ABCI_SERVER_URL=tcp://localhost:26662
    export ABCI_RPC_SERVER_URL=127.0.0.1:26661
    ./../../target/debug/node-template --base-path=$SUBSTRATE_NODE_2_HOME --chain=local --alice &> tmp/substrate_node_2_log.log &
    export SUBSTRATE_NODE_2_PID=$!

    unset ABCI_SERVER_URL
    unset DEFAULT_ABCI_RPC_URL
    unset ABCI_GENESIS_STATE_PATH
}

function clean_substrate_2() {
    echo "Clean Substrate node 2"
    ./../../target/debug/node-template purge-chain --base-path=$SUBSTRATE_NODE_2_HOME --chain=local
}

########

function start_cosmos_1() {
    echo "Setup cosmos application node 1"
    cp -R $HOME/.nsd $COSMOS_NODE_1_HOME
    nsd start --with-tendermint=false --transport=grpc --address=tcp://0.0.0.0:26660 --rpc.laddr=tcp://127.0.0.1:26659 --home=$COSMOS_NODE_1_HOME &> tmp/cosmos_node_1_log.log &
    export COSMOS_NODE_1_PID=$!
}

function clean_cosmos_1() {
    echo "Clean Cosmos node 1"
    nsd unsafe-reset-all --home=$COSMOS_NODE_1_HOME
}

##

function start_cosmos_2() {
    echo "Setup cosmos application node 2"
    cp -R $HOME/.nsd $COSMOS_NODE_2_HOME
    nsd start --with-tendermint=false --transport=grpc --address=tcp://0.0.0.0:26662 --rpc.laddr=tcp://127.0.0.1:26661 --home=$COSMOS_NODE_2_HOME &> tmp/cosmos_node_2_log.log &
    export COSMOS_NODE_2_PID=$!
}

function clean_cosmos_2() {
    echo "Clean Cosmos node 2"
    nsd unsafe-reset-all --home=$COSMOS_NODE_2_HOME
}


#######

function stop_substrate() {
    echo "Stop Substrate node"
    kill $SUBSTRATE_PID
}

function stop_cosmos() {
    echo "Stop Cosmos node"
    kill $COSMOS_PID
}

function insert_keys() {
    echo "Insert substrate accounts keys"
    curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["aura", "//Alice", "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"] }'
    curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["gran", "//Alice", "0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee"] }'

    curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["aura", "//Bob", "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"] }'
    curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["gran", "//Bob", "0xd17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69"] }'

    curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["aura", "//Charlie", "0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22"] }'
    curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["gran", "//Charlie", "0x439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f"] }'

    curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["aura", "//Dave", "0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20"] }'
    curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["gran", "//Dave", "0x5e639b43e0052c47447dac87d6fd2b6ec50bdd4d0f614e4299c665249bbd09d9"] }'
}

function start_all() {
    clean_tmp
    clean_cosmos
    start_cosmos
    sleep 2s
    clean_substrate
    start_substrate
}

function stop_all() {
    stop_substrate
    stop_cosmos
}

