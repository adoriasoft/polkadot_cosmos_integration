#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

## import
source ./testing_setup/test_utils.sh
source ./testing_setup/basic_setup.sh

## Run cosmos and substrate nodes
start_all
sleep 20s

## sync bug test

# Check the amounts
value=$(nscli q bank balances $(nscli keys show jack -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"100000000\"\n  denom: stake'
assert_eq "$value" "$expected"

value=$(nscli q bank balances $(nscli keys show alice -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"100000000\"\n  denom: stake'
assert_eq "$value" "$expected"

for i in {1..200}
do
    nscli tx send  $(nscli keys show jack -a) $(nscli keys show alice -a) 50000000stake --chain-id=namechain --from jack -y
done

for i in {1..200}
do
    nscli tx send  $(nscli keys show jack -a) $(nscli keys show alice -a) 50000000stake --chain-id=namechain --from jack -y
done
sleep 20s

# Check the amounts
value=$(nscli q bank balances $(nscli keys show jack -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"0\"\n  denom: stake'
assert_eq "$value" "$expected"

value=$(nscli q bank balances $(nscli keys show alice -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"200000000\"\n  denom: stake'
assert_eq "$value" "$expected"

test_passed "tx spamming test"

