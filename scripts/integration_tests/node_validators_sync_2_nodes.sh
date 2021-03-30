#!/bin/bash

#! This test runs two nodes of the substarte and corresponding to it two nodes of the cosmos-sdk
#! 1. Set up 2 cosmos validators using the `simd tx staking create-validator` command.
#! 2. Match fist cosmos validator to the substarte validator Bob, so as a result we expect that susbrate will change the validator list to the one Bob
#! 3. Match the same first cosmos validator to the another subsrate validator Alice, so as a result we expect that susbrate will change the validator list to the one Alice
#! 4. Math the second cosmos validator to the last substrate validator Bob, so as a result we expect that susbrate will change the validator list to the Bob and Alice validators

trap "exit" INT TERM ERR
trap "kill 0" EXIT

expect_validators_set_1="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_2="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
expect_validators_set_3="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
# same validators like in the first set but in the different order
expect_validators_set_4="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty@5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

cosmos_validator_pub_key1="0xa4f588be5bd917c0933d6fe1ac18d05b25dd5b27890327a57b9137b986736f15"
cosmos_validator_pub_key2="0x3fa902670c7e86c3426108fabf826dcd03578fbdd9efc78483e452a25c74e622"

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

sleep 20s

#insert new validator into the cosmos
simd tx staking create-validator \
 --amount=10000000stake \
 --pubkey=cosmosvalconspub1zcjduepq875syecv06rvxsnppratlqnde5p40raam8hu0pyru3f2yhr5uc3qpju33s \
 --moniker="alice validator" \
 --chain-id=test_chain \
 --from=alice \
 --commission-rate="0.10" \
 --commission-max-rate="0.20" \
 --commission-max-change-rate="0.01" \
 --min-self-delegation="1" \
 --gas-prices="0.025stake" \
 --node tcp://localhost:26659 \
 -y

simd tx staking create-validator \
 --amount=10000000stake \
 --pubkey=cosmosvalconspub1zcjduepq5n6c30jmmytupyeadls6cxxstvja6ke83ypj0ftmjymmnpnndu2s0793yf \
 --moniker="jack validator" \
 --chain-id=test_chain \
 --from=jack \
 --commission-rate="0.10" \
 --commission-max-rate="0.20" \
 --commission-max-change-rate="0.01" \
 --min-self-delegation="1" \
 --gas-prices="0.025stake" \
 --node tcp://localhost:26659 \
 -y

sleep 10s

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_1
node ./insert-cosmos-validator.app.js //Bob $cosmos_validator_pub_key1
sleep 30s

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_2

node ./insert-cosmos-validator.app.js //Alice $cosmos_validator_pub_key1
sleep 50s

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_3

node ./insert-cosmos-validator.app.js //Bob $cosmos_validator_pub_key2
sleep 50s

validators_set=$(node ./get-validators.app.js)
assert_eq "$validators_set" $expect_validators_set_1 $expect_validators_set_4

test_passed "node_validators_sync_2_nodes test passed"

