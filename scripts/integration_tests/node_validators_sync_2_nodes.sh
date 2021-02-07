#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

expect_validators_set_1="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_2="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_3="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

cosmos_validator_pub_key="0x576df7ddfdbd7d231d141c2d958bb69f9d84856a235afa618f09351395d25612"

## import
source ./testing_setup/test_utils.sh
source ./testing_setup/basic_setup.sh

## Start cosmos nodes
clean_cosmos_1
start_cosmos_1
sleep 2s
clean_cosmos_2
start_cosmos_2
sleep 2s

## Start substarte nodes
clean_substrate_1
start_substrate_1
sleep 5s
clean_substrate_2
start_substrate_2
sleep 5s

cd ../../node_testing_ui

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_1
node ./insert-cosmos-validator.app.js //Bob $cosmos_validator_pub_key
sleep 30s

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_2

node ./insert-cosmos-validator.app.js //Alice $cosmos_validator_pub_key
sleep 30s

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_3

stop_substrate_2

test_passed "node_validators_sync_2_nodes test passed"

