#!/usr/bin/env bash

 nsd unsafe-reset-all
 ./target/debug/node-template purge-chain --dev