#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

expect_validators_set_1="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_2="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_3="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
expected_validator_power="100"

cosmos_validator_pub_key="0x576df7ddfdbd7d231d141c2d958bb69f9d84856a235afa618f09351395d25612"

source ./testing_setup/basic_setup.sh
source ./testing_setup/test_utils.sh

start_all
sleep 5s

insert_keys

cd ../../node_testing_ui

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_1
node ./insert-cosmos-validator.app.js //Bob $cosmos_validator_pub_key
sleep 30s

validators_set=$(node ./get-validators.app.js)
validator_power=$(node ./get-validator-power.js //Bob)
assert_eq "$validators_set" $expect_validators_set_2
echo "The validator power must be equal to `100`"
assert_eq "$validator_power" $expected_validator_power

node ./insert-cosmos-validator.app.js //Alice $cosmos_validator_pub_key
sleep 30s

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_3

test_passed "node_validators_sync test passed"