#!/bin/bash

#! This test runs one node of the substarte and corresponding to it one cosmos node
#! 1. Set up 1 cosmos validators using the `nsd tx staking create-validator` command.
#! 2. Match fist cosmos validator to the substarte validator Bob, so as a result we expect that susbrate will change the validator list to the one Bob
#! 3. Match the same first cosmos validator to the another subsrate validator Alice, so as a result we expect that susbrate will change the validator list to the one Alice

trap "exit" INT TERM ERR
trap "kill 0" EXIT

expect_validators_set_1="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_2="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_3="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

# The node key from `$HOME/.nsd/config/priv_validator_key.json` pub_key=>value.
cosmos_validator_pub_key="0xa4f588be5bd917c0933d6fe1ac18d05b25dd5b27890327a57b9137b986736f15"

source ./testing_setup/basic_setup.sh
source ./testing_setup/test_utils.sh

start_all
sleep 20s

nsd tx staking create-validator \
 --amount=10000000stake \
 --pubkey=cosmosvalconspub1zcjduepq5n6c30jmmytupyeadls6cxxstvja6ke83ypj0ftmjymmnpnndu2s0793yf \
 --moniker="alex validator" \
 --chain-id=namechain \
 --from=alice \
 --commission-rate="0.10" \
 --commission-max-rate="0.20" \
 --commission-max-change-rate="0.01" \
 --min-self-delegation="1" \
 --gas-prices="0.025stake"

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

test_passed "node_validators_sync test passed"