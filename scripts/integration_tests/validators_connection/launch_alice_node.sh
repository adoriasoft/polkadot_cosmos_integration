# $1 = CHAIN_ID
function launch_alice_node() {
    echo 'Launch Alice node'
    export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
    echo $ABCI_GENESIS_STATE_PATH
    # Clean Alice node storage
    ./../../../target/debug/node-template purge-chain --base-path /tmp/alice --chain $1
    # Start Alice's node
    ./../../../target/debug/node-template \
        --base-path /tmp/alice \
        --chain $1 \
        --alice \
        --port 30333 \
        --ws-port 9945 \
        --rpc-port 9933 \
        --validator
        --node-key 0000000000000000000000000000000000000000000000000000000000000001
    export ALICE_NODE_PID=$!
    bash
}

launch_alice_node local