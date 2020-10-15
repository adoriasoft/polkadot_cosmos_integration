#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

function clean() {
    rm -rf ../tmp/*
}

function substrate_start() {
    echo "Reset Substrate"
    export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
    ./../../target/debug/node-template purge-chain --dev -y
    echo "Run Substrate"
    ./../../target/debug/node-template --dev &> tmp/substrate_log.log &
}

function comsos_start() {
    echo "Setup cosmos application"
    nsd unsafe-reset-all
    nsd start --with-tendermint=false --transport=grpc &> tmp/cosmos_log.log &
}


clean
comsos_start
sleep 1s
substrate_start

wait

