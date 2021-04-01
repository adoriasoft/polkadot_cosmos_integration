#!/usr/bin/env bash

rm -rf ~/.simapp

simd init test --chain-id test_chain

simd keys add jack --keyring-backend test
simd keys add alice --keyring-backend test
simd keys add alex --keyring-backend test

simd add-genesis-account $(simd keys show jack --keyring-backend test -a) 100000000stake
simd add-genesis-account $(simd keys show alice --keyring-backend test -a) 100000000stake
simd add-genesis-account $(simd keys show alex --keyring-backend test -a) 100000000stake

simd gentx alex 1000000stake --chain-id test_chain --keyring-backend test

simd collect-gentxs

echo "Validating genesis file..."
simd validate-genesis
