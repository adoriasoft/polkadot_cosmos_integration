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
    ./../../target/debug/node-template purge-chain --dev -y
}

function start_substrate() {
    echo "Run Substrate"
    ./../../target/debug/node-template --abci_genesis_state_path $HOME/.nsd/config/genesis.json --dev --bob &> tmp/substrate_log.log &
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
    ./../../target/debug/node-template --abci_genesis_state_path $HOME/.nsd/config/genesis.json --abci_server_url tcp://localhost:26660 --abci_rpc_url 127.0.0.1:26659 --base-path=$SUBSTRATE_NODE_1_HOME --chain=local --alice &> tmp/substrate_node_1_log.log &
    export SUBSTRATE_NODE_1_PID=$!
}

function clean_substrate_1() {
    echo "Clean Substrate node 1"
    ./../../target/debug/node-template purge-chain --base-path=$SUBSTRATE_NODE_1_HOME --chain=local
}

function stop_substrate_1() {
    echo "Stop Substrate node 1"
    kill $SUBSTRATE_NODE_1_PID
}

##

function start_substrate_2() {
    echo "Run Substrate node 2"
    ./../../target/debug/node-template --abci_genesis_state_path $HOME/.nsd/config/genesis.json --abci_server_url tcp://localhost:26662 --abci_rpc_url 127.0.0.1:26661 --base-path=$SUBSTRATE_NODE_2_HOME --chain=local --bob &> tmp/substrate_node_2_log.log &
    export SUBSTRATE_NODE_2_PID=$!
}

function clean_substrate_2() {
    echo "Clean Substrate node 2"
    ./../../target/debug/node-template purge-chain --base-path=$SUBSTRATE_NODE_2_HOME --chain=local
}

function stop_substrate_2() {
    echo "Stop Substrate node 2"
    kill $SUBSTRATE_NODE_2_PID
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

function stop_cosmos_1() {
    echo "Stop Cosmos node 1"
    kill $COSMOS_NODE_1_PID
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

function stop_cosmos_2() {
    echo "Stop Cosmos node 2"
    kill $COSMOS_NODE_2_PID
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

