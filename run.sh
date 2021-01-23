#!/usr/bin/env bash

trap "kill 0" EXIT

mkdir logs

nsd start --with-tendermint=false --transport=grpc &> logs/cosmos_log.log &
sleep 1s
./target/debug/node-template --dev