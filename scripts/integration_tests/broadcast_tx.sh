#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

## import
source ./testing_setup/test_utils.sh
source ./testing_setup/basic_setup.sh

## Run cosmos and substrate nodes
start_all
sleep 20s

## broadcast_tx_sync test (sync mode)
nscli tx nameservice buy-name jack.id 5nametoken --from jack --chain-id namechain -y --broadcast-mode sync
sleep 20s
nscli tx nameservice set-name jack.id jack_my --from jack --chain-id namechain -y --broadcast-mode sync
sleep 20s

## broadcast_tx_async test (async mode)
nscli tx nameservice buy-name alice.id 5nametoken --from alice --chain-id namechain -y --broadcast-mode async
sleep 20s
nscli tx nameservice set-name alice.id alice_my --from alice --chain-id namechain -y --broadcast-mode async
sleep 20s

jack_id=$(nscli query nameservice resolve jack.id)
alice_id=$(nscli query nameservice resolve alice.id)
assert_eq "$jack_id" "value: jack_my"
assert_eq "$alice_id" "value: alice_my"

test_passed "broadcast_tx_sync"
