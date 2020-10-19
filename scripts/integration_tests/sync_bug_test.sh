#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

##
source ./testing_setup/test_utils.sh

## Run cosmos and substrate nodes
./testing_setup/basic_setup.sh &
sleep 10s

## sync bug test

for i in {1..200}
do
    nscli tx nameservice buy-name jack.id 5nametoken --from jack --chain-id namechain -y
done