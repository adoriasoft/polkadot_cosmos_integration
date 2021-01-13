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

nscli tx nameservice buy-name jack.id 5nametoken --from jack --chain-id namechain -y
sleep 20s
nscli tx nameservice set-name jack.id hello_world --from jack --chain-id namechain -y
sleep 20s

value=$(nscli query nameservice resolve jack.id)
assert_eq "$value" "value: hello_world"

for i in {1..100}
do 
echo "Iteration " $i
stop_all

clean_tmp
start_cosmos
sleep 1s
start_substrate
sleep 5s

value=$(nscli query nameservice resolve jack.id)
assert_eq "$value" "value: hello_world"
done 

test_passed "stoping nodes test"