#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

function clean() {
    rm -rf tmp
    mkdir tmp
    touch tmp/substrate_log.log
    touch tmp/cosmos_log.log
}

function substrate_start() {
    echo "Reset Substrate"
    export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
    ./../../target/debug/node-template purge-chain --dev -y
    echo "Run Substrate"
    ./../../target/debug/node-template --dev &> tmp/substrate_log.log &
    SUBSTRATE_PID=$!
    echo "$SUBSTRATE_PID"
}

function comsos_start() {
    echo "Setup cosmos application"
    nsd unsafe-reset-all
    nsd start --with-tendermint=false --transport=grpc &> tmp/cosmos_log.log &
    COSMOS_PID=$!
    echo "$COSMOS_PID"
}

function stop_substrate() {
    echo "$SUBSTRATE_PID"
    kill $SUBSTRATE_PID
}

function stop_cosmos() {
    echo "$COSMOS_PID"
    kill $COSMOS_PID
}

function start_all() {
    clean
    comsos_start
    sleep 1s
    substrate_start
}

function stop_all() {
    stop_substrate
    stop_cosmos
}

