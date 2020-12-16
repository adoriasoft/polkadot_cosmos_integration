function stop_bob_node() {
    echo 'Stop Bob node'
    echo $BOB_NODE_PID
    kill -9 $BOB_NODE_PID
}

stop_bob_node