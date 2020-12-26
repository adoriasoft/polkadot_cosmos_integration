#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

# Launch Alice node with single initial validator = (Alice)
# Blocks must be produced
# Update validators list with two other validators on session 2 = (Bob, Dave)
# Blocks must not be produced
# Update validators list with single validator = (Alice)
# Blocks started produces again
# Each 30s block height comes +16 blocks.
source ../testing_setup/basic_setup.sh
source ../testing_setup/test_utils.sh
source ../nodes_setup/launch_alice_node.sh

clean_rocks_db
clean_tmp
launch_alice_node dev
sleep 10s

cd ../../../node_testing_ui

insert_alice_account=$(node ./tx.app.js //Alice 502f56364748755a72623872732f6b316f426f727863367679584d6c6e7a684a6d76374c6d6a454c4479733d)
insert_bob_account=$(node ./tx.app.js //Bob 536f6461436f6f6c)
insert_dave_account=$(node ./tx.app.js //Dave 4c6f76656c794d6f6e6b6579)
echo "After inserting Alice account tx hash $insert_alice_account"
echo "After inserting Bob account tx hash $insert_bob_account"
echo "After inserting Dave account tx hash $insert_dave_account"

chain_height_1=$(node ./block-info.app.js)
assert_eq "$chain_height_1" "13" "14" "15"
# Update staking on cosmos side
# to get fresh validators set.
sleep 30s

chain_height_2=$(node ./block-info.app.js)
assert_eq "$chain_height_2" "29" "30" "31"
# Update staking on cosmos side
# to get fresh validators set.
sleep 30s

chain_height_3=$(node ./block-info.app.js)
len_3=${#chain_height_3}
assert_eq "$chain_height_3" "45" "46" "47"

test_passed "Validator updates passed"