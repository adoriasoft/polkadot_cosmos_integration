#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

initial_validators_set="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
after_first_update_validators_set="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty@5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

# Launch Substrate node with single initial validator = (Alice)
# Check new validators length must be equal to 1
# Update validators list with two other validators = (Alice, Dave)
# Check new validators length must be equal to 2
# Update validators list with single validator = (Alice)
# Check new validators length must be equal to 1
source ./testing_setup/basic_setup.sh
source ./testing_setup/test_utils.sh

start_all
sleep 20s

cd ../../node_testing_ui

validators_set_1=$(node ./get-validators.app.js)
assert_eq "$validators_set_1" $initial_validators_set

insert_bob_account=$(node ./tx.app.js //Bob 0 0xa911e89ab8aec83f3c15701e1305486f47403a541659d20c3c43929cc31d34b9)
sleep 20s

validators_set_2=$(node ./get-validators.app.js)
assert_eq "$validators_set_2" $after_first_update_validators_set

test_passed "Cosmos/Substrate node sync test passed"