package main

import (
	"abci-grpc/src/abci"
	"abci-grpc/src/token"
	"context"
	"flag"
	"log"
	"net"
	"net/http"
	"sync"

	"github.com/golang/glog"
	"github.com/grpc-ecosystem/grpc-gateway/runtime"
	"google.golang.org/grpc"
)

const (
	grpc_port      = ":8081"
	grpc_http_port = ":8082"
)

var (
	// command-line options:
	// gRPC server endpoint
	grpcServerEndpoint = flag.String("grpc-server-endpoint", "localhost"+grpc_port, "gRPC server endpoint")
)

func Grpc_http_run() {
	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	// Register gRPC server endpoint
	// Note: Make sure the gRPC server is running properly and accessible
	mux := runtime.NewServeMux()
	opts := []grpc.DialOption{grpc.WithInsecure()}
	err := abci.RegisterAbciHandlerFromEndpoint(ctx, mux, *grpcServerEndpoint, opts)

	if err != nil {
		glog.Fatal(err)
		return
	}

	err = token.RegisterTokenHandlerFromEndpoint(ctx, mux, *grpcServerEndpoint, opts)

	log.Print("Grpc http server started")

	// Start HTTP server (and proxy calls to gRPC server endpoint)
	err = http.ListenAndServe(grpc_http_port, mux)

	if err != nil {
		glog.Fatal(err)
	}
}

func Grpc_run() {
	lis, err := net.Listen("tcp", grpc_port)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	tk := token.InitToken("token_data")
	defer tk.StopToken()

	s := grpc.NewServer()
	abci.RegisterAbciServer(s, &abci.ServerABCI{Token: tk})
	token.RegisterTokenServer(s, &token.ServerToken{Token: tk})

	log.Print("Grpc server started")

	err = s.Serve(lis)

	if err != nil {
		glog.Fatal(err)
	}
}

func main() {
	flag.Parse()
	defer glog.Flush()

	var wg sync.WaitGroup

	wg.Add(1)
	go func() {
		defer wg.Done()
		Grpc_run()
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		Grpc_http_run()
	}()

	wg.Wait()
}
