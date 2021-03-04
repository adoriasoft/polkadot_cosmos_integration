#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

##
source ./testing_setup/test_utils.sh
source ./testing_setup/basic_setup.sh

## Run cosmos and substrate nodes
start_all
sleep 20s

## stoping nodes test
# Check the amounts

value=$(nscli q bank balances $(nscli keys show jack -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"100000000\"\n  denom: stake'
assert_eq "$value" "$expected"

value=$(nscli q bank balances $(nscli keys show alice -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"100000000\"\n  denom: stake'
assert_eq "$value" "$expected"

# Send 50000000 stake tokens from Jack to Alice
nscli tx send  $(nscli keys show jack -a) $(nscli keys show alice -a) 50000000stake --chain-id=namechain --from jack -y
sleep 20s


# stoping nodes

for i in {1..100}
do 
echo "Iteration " $i
stop_all

clean_tmp
start_cosmos
sleep 1s
start_substrate
sleep 5s

# Check the amounts
value=$(nscli q bank balances $(nscli keys show jack -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"50000000\"\n  denom: stake'
assert_eq "$value" "$expected"

value=$(nscli q bank balances $(nscli keys show alice -a))
echo "$value"
expected=$'- amount: \"1000\"\n  denom: nametoken\n- amount: \"150000000\"\n  denom: stake'
assert_eq "$value" "$expected"

done 

test_passed "stoping nodes test"