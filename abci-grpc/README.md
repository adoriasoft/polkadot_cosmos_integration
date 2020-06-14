# Abci server with HTTP gateway implemented in go

Server for gRPC ABCI REST API.

## Generation of gRPC

### Install protobuf libraries

Install all of the protobuf libraries [here](https://github.com/grpc-ecosystem/grpc-gateway) and make sure to have valid $PATH that points on golang `bin/` folder.

### Code generation

To generate code for golang via protoc run next commands:

```sh
# Generate gRPC stub
protoc -I=./proto --go_out=plugins=grpc,paths=source_relative:./src/abci ./proto/abci.proto
# Generate reverse-proxy using protoc-gen-grpc-gateway
protoc -I=./proto --grpc-gateway_out=logtostderr=true,paths=source_relative:./src/abci ./proto/abci.proto
```

### Run and execute

Run via `docker-compose`:

```sh
docker-compose up
```

### Send HTTP requests

```sh
curl -H 'Content-Type: application/json' -XPOST -d '{"tx": "hello from curl!!!!"}' http://localhost:8082/abci/v1/CheckTx
curl -H 'Content-Type: application/json' -XPOST -d '{"tx": "hello from curl!!!!"}' http://localhost:8082/abci/v1/DeliverTx
```
