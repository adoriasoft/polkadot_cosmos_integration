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
value=$(simd q bank balances $(simd keys show jack -a --keyring-backend test) --node tcp://localhost:26659)
echo "$value"
expected=$'balances:\n- amount: \"100000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

value=$(simd q bank balances $(simd keys show alice -a --keyring-backend test) --node tcp://localhost:26659)
echo "$value"
expected=$'balances:\n- amount: \"100000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

# Check the amounts on the cosmos node 2
value=$(simd q bank balances $(simd keys show jack -a --keyring-backend test) --node tcp://localhost:26661)
echo "$value"
expected=$'balances:\n- amount: \"100000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

value=$(simd q bank balances $(simd keys show alice -a --keyring-backend test) --node tcp://localhost:26661)
echo "$value"
expected=$'balances:\n- amount: \"100000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

# Send 25000000 stake tokens from Jack to Alice
simd tx bank send  $(simd keys show jack -a --keyring-backend test) $(simd keys show alice -a --keyring-backend test) 25000000stake --chain-id=test_chain --from jack --node tcp://localhost:26659 --keyring-backend test -y
sleep 20s

simd tx bank send  $(simd keys show jack -a --keyring-backend test) $(simd keys show alice -a --keyring-backend test) 25000000stake --chain-id=test_chain --from jack --node tcp://localhost:26661 --keyring-backend test -y
sleep 20s

# Check the amounts on the cosmos node 1
value=$(simd q bank balances $(simd keys show jack -a --keyring-backend test) --node tcp://localhost:26659)
echo "$value"
expected=$'balances:\n- amount: \"50000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

value=$(simd q bank balances $(simd keys show alice -a --keyring-backend test) --node tcp://localhost:26659)
echo "$value"
expected=$'balances:\n- amount: \"150000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

# Check the amounts on the cosmos node 2
value=$(simd q bank balances $(simd keys show jack -a --keyring-backend test) --node tcp://localhost:26661)
echo "$value"
expected=$'balances:\n- amount: \"50000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

value=$(simd q bank balances $(simd keys show alice -a --keyring-backend test) --node tcp://localhost:26661)
echo "$value"
expected=$'balances:\n- amount: \"150000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

test_passed "basic test 2 nodes"

