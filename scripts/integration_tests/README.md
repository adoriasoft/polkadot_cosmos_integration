## Prapare local environment for running tests

1. Make sure Go installed on your machine.
   Prepare your Comsos environment as described in https://github.com/adoriasoft/cosmos-sdk/tree/feature/add_nameservice/simapp.

2. Built Substrate node from the root.

3. Make sure you have Node.js installed on your machine.

```sh
cd ../../node_testing_ui
yarn install
```

## Run tests

You can run each tests separetelly:

```sh
./basic_test.sh
./basic_test_2_nodes.sh
./broadcast_tx.sh
./docker_test.sh
./stoping_nodes_test.sh
./tx_spamming_test.sh
./node_validators_sync.sh
./node_validators_sync_2_nodes.sh
./cosmos_validator_rewards_test.sh
```

Each of this test will runs an instance of the substrate node with the cosmos node and interacts with this nodes via the comsos cli (nscli) or via rpc. You can find logs after execution of the each test in the 'tmp' directory.
