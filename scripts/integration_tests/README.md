# Integration tests

Before run this tests you should have been already built substrate node and cosmos node (nsd) and cosmos cli (nscli).
For these just follow the instructions from the root directory of this project.\
You can run each tests separetelly:
```sh
./basic_test.sh
./batch_tests.sh
./broadcast_tx.sh
./docker_test.sh
./stoping_nodes_test.sh
./tx_spamming_test.sh
```
Each of this test will runs an instance of the substrate node with the cosmos node and interacts with this nodes via the comsos cli (nscli) or via rpc. You can find logs after execution of the each test in the 'tmp' directory. 
