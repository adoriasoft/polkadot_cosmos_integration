package abci

import (
	"abci-grpc/src/token"
	context "context"
	"flag"
	"log"
	"net"
	"net/http"

	"github.com/boltdb/bolt"
	"github.com/golang/glog"
	"github.com/grpc-ecosystem/grpc-gateway/runtime"
	grpc "google.golang.org/grpc"
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
	UnimplementedAbciServer

	token token.Token
}

func (s *server) InitChain(ctx context.Context, in *EmptyMessage) (*EmptyMessage, error) {
	log.Print("received InitChain()")
	return &EmptyMessage{}, nil
}

func (s *server) CheckTx(ctx context.Context, in *CheckTxRequest) (*EmptyMessage, error) {
	log.Printf("Received CheckTx(), tx: %s", string(in.Tx))
	return &EmptyMessage{}, nil
}

func (s *server) DeliverTx(ctx context.Context, in *DeliverTxRequest) (*EmptyMessage, error) {
	log.Printf("Received DeliverTx(), tx: %s", string(in.Tx))
	return &EmptyMessage{}, nil
}

func (s *server) OnInitialize(ctx context.Context, in *BlockMessage) (*EmptyMessage, error) {
	log.Printf("Received OnInitialize(), block height: %d", in.Height)
	return &EmptyMessage{}, nil
}

func (s *server) OnFinilize(ctx context.Context, in *BlockMessage) (*EmptyMessage, error) {
	log.Printf("Received OnFinilize(), block height: %d", in.Height)
	return &EmptyMessage{}, nil
}

func (s *server) Commit(ctx context.Context, in *BlockMessage) (*EmptyMessage, error) {
	log.Printf("Received Commit(), block height: %d", in.Height)
	return &EmptyMessage{}, nil
}

func Grpc_http_run() {
	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	// Register gRPC server endpoint
	// Note: Make sure the gRPC server is running properly and accessible
	mux := runtime.NewServeMux()
	opts := []grpc.DialOption{grpc.WithInsecure()}
	err := RegisterAbciHandlerFromEndpoint(ctx, mux, *grpcServerEndpoint, opts)
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

func Grpc_run() {
	lis, err := net.Listen("tcp", grpc_port)
	if err != nil {
		log.Fatal("failed to listen: %v", err)
	}

	tk := token.InitToken()
	defer tk.StopToken()

	s := grpc.NewServer()
	RegisterAbciServer(s, &server{token: tk})

	log.Print("Grpc server started")
	// Open the my.db data file in your current directory.
	// It will be created if it doesn't exist.
	db, err := bolt.Open("my.db", 0600, nil)
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	err = s.Serve(lis)

	if err != nil {
		glog.Fatal(err)
	}
}
