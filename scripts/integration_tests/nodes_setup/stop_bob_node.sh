function stop_bob_node() {
    echo 'Stop Bob node with PID' $1
    kill -9 $1
}

stop_bob_node $BOB_NODE_PID