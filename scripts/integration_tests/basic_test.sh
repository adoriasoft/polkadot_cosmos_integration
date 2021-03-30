#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

## import
source ./testing_setup/test_utils.sh
source ./testing_setup/basic_setup.sh

## Run cosmos and substrate nodes
start_all
sleep 20s

# Check the amounts
value=$(simd q bank balances $(simd keys show jack -a --keyring-backend test))
echo "$value"
expected=$'balances:\n- amount: \"100000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

value=$(simd q bank balances $(simd keys show alice -a --keyring-backend test))
echo "$value"
expected=$'balances:\n- amount: \"100000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

# Send 50000000 stake tokens from Jack to Alice
simd tx bank send  $(simd keys show jack -a --keyring-backend test) $(simd keys show alice -a --keyring-backend test) 50000000stake --chain-id=test_chain --from jack --keyring-backend test -y
sleep 20s

# Check the amounts
value=$(simd q bank balances $(simd keys show jack -a --keyring-backend test))
echo "$value"
expected=$'balances:\n- amount: \"50000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

value=$(simd q bank balances $(simd keys show alice -a --keyring-backend test))
echo "$value"
expected=$'balances:\n- amount: \"150000000\"\n  denom: stake\npagination:\n  next_key: null\n  total: \"0\"'
assert_eq "$value" "$expected"

test_passed "basic test"