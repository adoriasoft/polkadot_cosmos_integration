FROM jaegertracing/protobuf AS proto-builder

COPY ./proto ./proto
RUN mkdir -p /res
RUN protoc-wrapper -I=./proto --go_out=plugins=grpc,paths=source_relative:/res ./proto/token.proto
RUN protoc-wrapper -I=./proto --grpc-gateway_out=logtostderr=true,paths=source_relative:/res ./proto/token.proto

FROM golang:1.14-alpine AS builder

WORKDIR /app
COPY . /app
COPY --from=proto-builder /res ./src/token
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags="-w -s" -o ./app ./src/server/

FROM scratch
COPY --from=builder /app/app /app
ENTRYPOINT ["/app"]
