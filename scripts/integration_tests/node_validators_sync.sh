#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

expect_validators_set_1="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_2="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_3="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

source ./testing_setup/basic_setup.sh
source ./testing_setup/test_utils.sh

start_all
sleep 5s

insert_keys

cd ../../node_testing_ui

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_1
node ./insert-cosmos-validator.app.js //Bob 0 0xa911e89ab8aec83f3c15701e1305486f47403a541659d20c3c43929cc31d34b9
sleep 30s

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_2

node ./insert-cosmos-validator.app.js //Alice 0 0xa911e89ab8aec83f3c15701e1305486f47403a541659d20c3c43929cc31d34b9
sleep 30s

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_3

test_passed "node_validators_sync test passed"