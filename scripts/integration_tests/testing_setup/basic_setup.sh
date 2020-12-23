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
    rm -rf abci_storage_rocksdb
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

