#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

initial_validators_set="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY@5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
after_first_update_validators_set="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"

# Launch Substrate node with single initial validator = (Alice)
# Check new validators length must be equal to 1
# Update validators list with two other validators = (Alice, Dave)
# Check new validators length must be equal to 2
# Update validators list with single validator = (Alice)
# Check new validators length must be equal to 1
source ./testing_setup/basic_setup.sh
source ./testing_setup/test_utils.sh

start_all
sleep 5s

curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["aura", "//Alice", "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"] }'
curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["gran", "//Alice", "0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee"] }'

curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["aura", "//Bob", "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"] }'
curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["gran", "//Bob", "0xd17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69"] }'

cd ../../node_testing_ui

validators_set_1=$(node ./get-validators.app.js)
assert_eq "$validators_set_1" $initial_validators_set
insert_bob_account=$(node ./insert-cosmos-validator.app.js //Bob 0 0xa911e89ab8aec83f3c15701e1305486f47403a541659d20c3c43929cc31d34b9)
sleep 30s

validators_set_2=$(node ./get-validators.app.js)
assert_eq "$validators_set_2" $after_first_update_validators_set

test_passed "Cosmos/Substrate node sync test passed"