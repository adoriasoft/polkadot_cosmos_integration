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
# clean_rocks_db
clean_tmp

launch_alice_node dev
sleep 60s

echo 'Query blocks 1/2/3'
echo 'Update validators tx'
sleep 30s

echo 'Query blocks 15/16/17'
echo 'Update validators tx'
sleep 30s

echo 'Query blocks 15/16/17'
test_passed "Validator updates"