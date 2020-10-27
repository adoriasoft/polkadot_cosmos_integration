#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

##
source ./testing_setup/test_utils.sh

## Run cosmos and substrate nodes
./testing_setup/basic_setup.sh &
sleep 20s


## basic test
nscli tx nameservice buy-name jack.id 5nametoken --from jack --chain-id namechain -y
sleep 10s
nscli tx nameservice set-name jack.id hello_world --from jack --chain-id namechain -y
sleep 10s

value=$(nscli query nameservice resolve jack.id)
assert_eq "$value" "value: hello_world"

nscli tx nameservice set-name jack.id hello_universe --from jack --chain-id namechain -y
sleep 20s

value=$(nscli query nameservice resolve jack.id)
assert_eq "$value" "value: hello_universe"

test_passed "basic test"