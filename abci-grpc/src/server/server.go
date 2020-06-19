package main

import (
	"abci-server/src/abci"
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

type server struct {
	abci.UnimplementedAbciServer
}

func (s *server) InitChain(ctx context.Context, in *abci.EmptyMessage) (*abci.EmptyMessage, error) {
	log.Print("received InitChain()")
	return &abci.EmptyMessage{}, nil
}

func (s *server) CheckTx(ctx context.Context, in *abci.CheckTxRequest) (*abci.EmptyMessage, error) {
	log.Printf("Received CheckTx(), tx: %s", string(in.Tx))
	return &abci.EmptyMessage{}, nil
}

func (s *server) DeliverTx(ctx context.Context, in *abci.DeliverTxRequest) (*abci.EmptyMessage, error) {
	log.Printf("Received DeliverTx(), tx: %s", string(in.Tx))
	return &abci.EmptyMessage{}, nil
}

func (s *server) OnInitialize(ctx context.Context, in *abci.BlockMessage) (*abci.EmptyMessage, error) {
	log.Printf("Received OnInitialize(), block height: %d", in.Height)
	return &abci.EmptyMessage{}, nil
}

func (s *server) OnFinilize(ctx context.Context, in *abci.BlockMessage) (*abci.EmptyMessage, error) {
	log.Printf("Received OnFinilize(), block height: %d", in.Height)
	return &abci.EmptyMessage{}, nil
}

func (s *server) Commit(ctx context.Context, in *abci.BlockMessage) (*abci.EmptyMessage, error) {
	log.Printf("Received Commit(), block height: %d", in.Height)
	return &abci.EmptyMessage{}, nil
}

func (s *server) Echo(ctx context.Context, in *abci.EmptyMessage) (*abci.EmptyMessage, error) {
	log.Printf("Received Echo()")
	log.Printf("___________________")
	return &abci.EmptyMessage{}, nil
}

func grpc_http_run() {
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

	log.Print("Grpc http server started")

	// Start HTTP server (and proxy calls to gRPC server endpoint)
	err = http.ListenAndServe(grpc_http_port, mux)

	if err != nil {
		glog.Fatal(err)
	}
}

func grpc_run() {
	lis, err := net.Listen("tcp", grpc_port)
	if err != nil {
		log.Fatal("failed to listen: %v", err)
	}

	s := grpc.NewServer()
	abci.RegisterAbciServer(s, &server{})

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
		grpc_run()
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		grpc_http_run()
	}()

	wg.Wait()
}
