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

To start the substarte node with token implementation first start token application

```
cd src/server
go run .
```

Then you can run the substrate node

```
cd ../../../substrate
cargo build
./target/debug/node-template --dev
```

Great!!! Now you have already started substarte node with the token application.
## Account generation

To generate a key pair for a new account open src/generate_tx/generate_tx.go, comment line with the token.GenerateTransactionMessage function and public keys, specify the account name for the token.GenerateKeyPairForAccount function. This function will print the private key that you should save somewhere, and message for transaction that should be broadcast to the substrate network.

The code will look like

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

ATTENTION!!!! SAVE THE PRIVATE KEY THAT WILL BE PRINTED, IT CAN`T BE RECOVERED

## Tokens sending

To generate transactions to send tokens from one account to another, open src/generate_tx/generate_tx.go, comment line with the token.GenerateKeyPairForAccount, specify parameters
FROM - sender`s name,
TO - receiver's name,
AMOUNT - number of tokens,
PRIVATE_KEY - sender's private key,
NONCE - the number of transaction sent by the sender
for the token.GenerateTransactionMessage function. This function will print the message for transaction that should be broadcast to the substrate network, that we will look later.

The code will look like

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

To broadcast transaction to the substrate net and further execute the token logic you should ffollow the next steps

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

To get the account info with the balance and current nonce, that must be specified for transaction generation, execute:
```
curl -H 'Content-Type: application/json' -XPOST -d '{"account_name": "Alice"}' http://localhost:8082/token/v1/GetAccountInfo
```

## Run Tests

```
cd src/token
go test
```










