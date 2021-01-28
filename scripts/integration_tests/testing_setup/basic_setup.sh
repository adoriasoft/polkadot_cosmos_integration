#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

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

