#!/bin/bash

cp -r ./abci_storage_rocksdb ./../../../
rm -rf ./abci_storage_rocksdb
# Start root Alice node
./launch_alice_node.sh &
# check that blocks do not finalized

# Boot Bob node to it
./launch_bob_node.sh
# check if blocks are finalized

# Kill Bob node
# ./stop_bob_node.sh
# check that blocks do not finalized

# Boot Bob Node to Alice node again
# ./launch_bob_node.sh
# check if blocks are finalized
