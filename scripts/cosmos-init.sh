#!/usr/bin/env bash

rm -rf ~/.simapp

simd init test --chain-id test_chain

simd keys add jack
simd keys add alice
simd keys add alex

simd add-genesis-account $(simd keys show jack -a) 100000000stake
simd add-genesis-account $(simd keys show alice -a) 100000000stake
simd add-genesis-account $(simd keys show alex -a) 100000000stake

simd gentx alex 1000000stake --chain-id test_chain

simd collect-gentxs

echo "Validating genesis file..."
simd validate-genesis
