# $1 = CHAIN_ID
function launch_alice_node() {
    echo 'Launch Alice node'
    export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
    echo $ABCI_GENESIS_STATE_PATH
    # Clean Alice node storage
    ./../../../target/debug/node-template purge-chain --base-path /tmp/alice --chain $1 -y
    # Start Alice's node
    ./../../../target/debug/node-template \
        --base-path /tmp/alice \
        --chain $1 \
        --alice \
        --port 30333 \
        --ws-port 9944 \
        --rpc-port 9933 \
        --validator &
    export ALICE_NODE_PID=$!
}

launch_alice_node dev