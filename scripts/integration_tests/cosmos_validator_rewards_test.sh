#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

expect_validators_set_1="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_2="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_3="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

cosmos_validator_pub_key="0x576df7ddfdbd7d231d141c2d958bb69f9d84856a235afa618f09351395d25612"

source ./testing_setup/basic_setup.sh
source ./testing_setup/test_utils.sh

start_all
sleep 10s

insert_keys

cd ../../node_testing_ui

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_1

sleep 30s

# withdraw rewards
nscli tx distribution withdraw-all-rewards --chain-id=namechain --from=$(nscli keys show jack -a) -y
sleep 5s

value=$(nscli q bank balances $(nscli keys show jack -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"0\"\n  denom: stake'
assert_eq "$value" "$expected"

node ./insert-cosmos-validator.app.js //Bob $cosmos_validator_pub_key
sleep 30s

# withdraw rewards
nscli tx distribution withdraw-all-rewards --chain-id=namechain --from=$(nscli keys show jack -a) -y
sleep 5s

value=$(nscli q bank balances $(nscli keys show jack -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"0\"\n  denom: stake'
assert_ne "$value" "$expected"

test_passed "cosmos_validator_rewards_test test passed"