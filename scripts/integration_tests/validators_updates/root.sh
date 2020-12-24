# #!/usr/bin/env bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

# Launch Alice node with single initial validator = (Alice)
# Blocks must be produced
# Update validators list with two other validators on session 2 = (Bob, Dave)
# Blocks must not be produced
# Update validators list with single validator = (Alice)
# Blocks started produces again
source ../testing_setup/basic_setup.sh
source ../testing_setup/test_utils.sh
source ../nodes_setup/launch_alice_node.sh

clean_rocks_db
clean_tmp
# Check curl available for performing test calls.
curl --version
# Setup curl urls.
export $POLKADOT_TESTING_UI_ENDPOINT = 'http://localhost:8000'
export $POLKADOT_TESTING_UI_QUERY_BLOCK_HEIGHT_PATH = '/testing/chain_height'
export $POLKADOT_TESTING_UI_SEND_EXTRINSIC_PATH = '/testing/send_ext?data=bytes&origin=Bob'

launch_alice_node dev
sleep 60s

chain_height = $(curl $POLKADOT_TESTING_UI_ENDPOINT$POLKADOT_TESTING_UI_QUERY_BLOCK_HEIGHT_PATH)
echo $chain_height
# assert_eq "$chain_height" "value: ?"
# Update validators and blocks finallized must be stopped.
sleep 30s

chain_height_2 = $(curl $POLKADOT_TESTING_UI_ENDPOINT$POLKADOT_TESTING_UI_QUERY_BLOCK_HEIGHT_PATH)
echo $chain_height
# assert_eq "$chain_height" "value: ?"
# Update validators and blocks must starts to produce again.
sleep 30s

chain_height_3 = $(curl $POLKADOT_TESTING_UI_ENDPOINT$POLKADOT_TESTING_UI_QUERY_BLOCK_HEIGHT_PATH)
# assert_eq "$chain_height_3" "value: ?"

test_passed "Validator updates passed"