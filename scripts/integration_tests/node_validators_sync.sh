#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

# Launch Substrate node with single initial validator = (Alice)
# Check new validators length must be equal to 1
# Update validators list with two other validators = (Alice, Dave)
# Check new validators length must be equal to 2
# Update validators list with single validator = (Alice)
# Check new validators length must be equal to 1
# NOTE Each 30s block height comes +16 blocks.
source ./testing_setup/basic_setup.sh
source ./testing_setup/test_utils.sh

start_all
sleep 20s

cd ../../node_testing_ui

insert_alice_account=$(node ./tx.app.js //Alice 0 dacf6e056bbefeb9333f35aec1a0a4c507afc4ce17552e0409fc72cf7e728bf0)
insert_bob_account=$(node ./tx.app.js //Bob 0 e0c4396a2138cecd48aa42d4dc2d2f587518859720e34017e24ff096c81c2c26)
echo "After inserting Alice account tx hash $insert_alice_account"
echo "After inserting Bob account tx hash $insert_bob_account"

validators_count_1=$(node ./get-validators-count.app.js)
assert_eq "$validators_count_1" "1"
# TODO staking on cosmos side
# to get fresh validators set.
sleep 20s

validators_count_2=$(node ./get-validators-count.app.js)
assert_eq "$validators_count_2" "2"
# TODO staking on cosmos side
# to get fresh validators set.
sleep 20s

validators_count_3=$(node ./get-validators-count.app.js)
assert_eq "$validators_count_3" "1"

test_passed "Cosmos/Substrate node sync test passed"