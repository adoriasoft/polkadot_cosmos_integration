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
curl -H 'Content-Type: application/json' -XPOST -d '{"tx": [104,101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 99, 117, 114, 108, 33, 33, 33]}' http://localhost:8082/abci/v1/CheckTx
curl -H 'Content-Type: application/json' -XPOST -d '{"tx": [104,101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 99, 117, 114, 108, 33, 33, 33]}' http://localhost:8082/abci/v1/DeliverTx
```
[104,101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 99, 117, 114, 108, 33, 33, 33] - byte array encoded message "hello from curl!!!"

PATH="/home/leshiy/.cargo/bin:/home/leshiy/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/local/sbin:/usr/lib/jvm/default/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/var/lib/snapd/snap/bin"
