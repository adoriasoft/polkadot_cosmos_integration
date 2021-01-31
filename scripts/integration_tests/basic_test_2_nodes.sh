#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

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

sleep 20s

## basic test
nscli tx nameservice buy-name jack.id 5nametoken --node tcp://localhost:26659 --from jack --chain-id namechain -y
sleep 20s
nscli tx nameservice set-name jack.id hello_world --node tcp://localhost:26661 --from jack --chain-id namechain -y
sleep 20s

value=$(nscli query nameservice resolve jack.id --node tcp://localhost:26659)
assert_eq "$value" "value: hello_world"
value=$(nscli query nameservice resolve jack.id --node tcp://localhost:26661)
assert_eq "$value" "value: hello_world"

nscli tx nameservice set-name jack.id hello_universe --node tcp://localhost:26659 --from jack --chain-id namechain -y
sleep 20s

value=$(nscli query nameservice resolve jack.id --node tcp://localhost:26659)
assert_eq "$value" "value: hello_universe"
value=$(nscli query nameservice resolve jack.id --node tcp://localhost:26661)
assert_eq "$value" "value: hello_universe"

test_passed "basic test 2 nodes"

