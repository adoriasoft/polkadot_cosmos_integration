# Abci server with HTTP gateway implemented in go

Server for gRPC ABCI REST API.

## Generation of gRPC

### Install protobuf libraries

Install all of the protobuf libraries [here](https://grpc.io/docs/languages/go/quickstart/) and make sure to have valid $PATH that points on golang `bin/` folder.

### Code generation

To generate code for golang via protoc run next commands:

```sh
# Generate gRPC stub for the abci
protoc -I=./proto --go_out=plugins=grpc,paths=source_relative:./src/abci ./proto/abci.proto
# Generate reverse-proxy using protoc-gen-grpc-gateway for the abci
protoc -I=./proto --grpc-gateway_out=logtostderr=true,paths=source_relative:./src/abci ./proto/abci.proto

# Generate gRPC stub for the token
protoc -I=./proto --go_out=plugins=grpc,paths=source_relative:./src/token ./proto/token.proto
# Generate reverse-proxy using protoc-gen-grpc-gateway for the token
protoc -I=./proto --grpc-gateway_out=logtostderr=true,paths=source_relative:./src/token ./proto/token.proto
```

### Send HTTP requests

```sh
curl -H 'Content-Type: application/json' -XPOST -d '{"account_name": "Alice"}' http://localhost:8082/token/v1/GetAccountInfo
```

### Build and publish docker image

Run these commands:

```sh
docker build -t andoriasoft/cosmos-node:latest .
docker push andoriasoft/cosmos-node:latest
```

## TOKEN Tutorial

Here is introduced a basic tutorial how to communicate with token.
How to generate transactions, create new account, send tokens to the another account etc.

### Prepare environment

To start the substarte node with token realisation first start token application

```
cd src/server
go run .
```

Than you can run the substrate node

```
cd ../../../substrate
cargo build
./target/debug/node-template --dev
```

Great!!! Now you have already started substarte node with the token application.
## Account generation

To generate account open src/generate_tx/generate_tx.go comment line with the token.GenerateTransactionMessage function and public keys,
specify the account name for the token.GenerateKeyPairForAccount function, this function will print the private key that you should to save somewhere and transaction that you have to broadcast to the substrate network

Here is the code that you should have to execute

```
package main

import (
	"abci-grpc/src/token"
)

func main() {
	token.GenerateKeyPairForAccount("Alex")
}
```

Execute

```
cd src/generate_tx
go run .
```

ATTENTION!!!! SAVE THE PRIVATE KEY THAT WILL BE PRINTED, IF NOT WE NOT ABLE TO RESTORE IT

## Tokens sending

To generate transactions for the tokens sending open src/generate_tx/generate_tx.go comment line with the token.GenerateKeyPairForAccount,
specify the account name FROM, TO, AMOUNT, PRIVATE_KEY, NONCE for the token.GenerateTransactionMessage function, this function will print the transaction that you have to broadcast to the substrate network, that we will look later.

Here is the code that you should have to execute

```
package main

import (
	"abci-grpc/src/token"
)

func main() {
	alice_private_key := "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgnmlvaImLkJlJXvu7vQ7t4Y6rqH/5jVsyuTa6B5vGC7KhRANCAAQTCnjgKkLm/7X9lRF2R+04RubrNk4Z5i6nRQkBGWICHNmwgITyEI5I6NUNtHN71zrP0DPV8m6G7GYADX1O4WHw"

	token.GenerateTransactionMessage("Alice", "Alex", 1000, alice_private_key, 0)
}

```

Execute

```
cd src/generate_tx
go run .
```

ATTENTION!!!! SAVE THE PRIVATE KEY THAT WILL BE PRINTED, IF NOT WE NOT ABLE TO RESTORE IT

## Broadcast transactions to the substrate net

To broadcast transaction to the substarte net and further execute the token logic you should follow to the next steps

1. Go to the https://polkadot.js.org/apps/
2. Go to the 'Settings' -> 'Developer' and replace the code to the next one
```
{
  "String": "Vec<u8>",
  "TxMessage": {
    "tx": "String"
  }
}
```
3. Go to the 'Extrinsics', choose the 'abciDirect' extrinsic
4. In the field 'tx: Text' put your message for the account generation or for the token transfering.
5. Submit transaction and send !!!

## Get the account info

To get the account info with the balance and current nonce, that is important to specify while transaction generation, execute:
```
curl -H 'Content-Type: application/json' -XPOST -d '{"account_name": "Alice"}' http://localhost:8082/token/v1/GetAccountInfo
```










