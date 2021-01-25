#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

# Launch Substrate node with single initial validator = (Alice)
# Blocks must be produced
# Update validators list with two other validators = (Bob, Dave)
# Blocks must not be produced
# Update validators list with single validator = (Alice)
# Blocks started produces again
# Each 30s block height comes +16 blocks.
source ./testing_setup/basic_setup.sh
source ./testing_setup/test_utils.sh

start_all
sleep 20s

cd ../../node_testing_ui

insert_alice_account=$(node ./tx.app.js //Alice dacf6e056bbefeb9333f35aec1a0a4c507afc4ce17552e0409fc72cf7e728bf0)
insert_bob_account=$(node ./tx.app.js //Bob e0c4396a2138cecd48aa42d4dc2d2f587518859720e34017e24ff096c81c2c26)
# insert_dave_account=$(node ./tx.app.js //Dave 4c6f76656c794d6f6e6b6579)
echo "After inserting Alice account tx hash $insert_alice_account"
echo "After inserting Bob account tx hash $insert_bob_account"
# echo "After inserting Dave account tx hash $insert_dave_account"

chain_height_1=$(node ./block-info.app.js)
assert_eq "$chain_height_1" "14" "15" "16"
# TODO staking on cosmos side
# to get fresh validators set.
sleep 30s

# chain_height_2=$(node ./block-info.app.js)
# assert_eq "$chain_height_2" "29" "30" "31"
# TODO staking on cosmos side
# to get fresh validators set.
# sleep 30s

# chain_height_3=$(node ./block-info.app.js)
# len_3=${#chain_height_3}
# assert_eq "$chain_height_3" "45" "46" "47"

test_passed "Cosmos/Substrate node sync test passed"