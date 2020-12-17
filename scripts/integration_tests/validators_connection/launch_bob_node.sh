# $1 = ALICE_PEER_ID
# $2 = CHAIN_ID
function launch_bob_node() {
    echo 'Launch Bob node'
    export ABCI_GENESIS_STATE_PATH=$HOME/.nsd/config/genesis.json
    echo $ABCI_GENESIS_STATE_PATH
    # Clean Bob node storage
    ./../../../target/debug/node-template purge-chain --base-path /tmp/bob --chain $1 -y
    # Start Bob node
    ./../../../target/debug/node-template \
        --base-path /tmp/bob \
        --chain $2 \
        --bob \
        --port 30334 \
        --ws-port 9946 \
        --rpc-port 9934 \
        --validator \
        --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$2
    export BOB_NODE_PID=$!
}

launch_bob_node dev 12D3KooWKWmdYGALKJha4CPy6H8HmGfC1S35W4dsqtsTD37v9Unb

sleep 60s

echo 'Bob node up with PID' $BOB_NODE_PID

# kill $BOB_NODE_PID