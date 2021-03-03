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

# Check the amounts on the cosmos node 1
value=$(nscli q bank balances $(nscli keys show jack -a) --node tcp://localhost:26659)
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"100000000\"\n  denom: stake'
assert_eq "$value" "$expected"

value=$(nscli q bank balances $(nscli keys show alice -a) --node tcp://localhost:26659)
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"100000000\"\n  denom: stake'
assert_eq "$value" "$expected"

# Check the amounts on the cosmos node 2
value=$(nscli q bank balances $(nscli keys show jack -a) --node tcp://localhost:26661)
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"100000000\"\n  denom: stake'
assert_eq "$value" "$expected"

value=$(nscli q bank balances $(nscli keys show alice -a) --node tcp://localhost:26661)
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"100000000\"\n  denom: stake'
assert_eq "$value" "$expected"

# Send 25000000 stake tokens from Jack to Alice
nscli tx send  $(nscli keys show jack -a) $(nscli keys show alice -a) 25000000stake --chain-id=namechain --from jack --node tcp://localhost:26659 -y
sleep 20s

nscli tx send  $(nscli keys show jack -a) $(nscli keys show alice -a) 25000000stake --chain-id=namechain --from jack --node tcp://localhost:26661 -y
sleep 20s

# Check the amounts on the cosmos node 1
value=$(nscli q bank balances $(nscli keys show jack -a) --node tcp://localhost:26659)
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"50000000\"\n  denom: stake'
assert_eq "$value" "$expected"

value=$(nscli q bank balances $(nscli keys show alice -a) --node tcp://localhost:26659)
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"150000000\"\n  denom: stake'
assert_eq "$value" "$expected"

# Check the amounts on the cosmos node 2
value=$(nscli q bank balances $(nscli keys show jack -a) --node tcp://localhost:26661)
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"50000000\"\n  denom: stake'
assert_eq "$value" "$expected"

value=$(nscli q bank balances $(nscli keys show alice -a) --node tcp://localhost:26661)
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"150000000\"\n  denom: stake'
assert_eq "$value" "$expected"

test_passed "basic test 2 nodes"

