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

stop_all
sleep 20s


test_passed "stoping nodes test"