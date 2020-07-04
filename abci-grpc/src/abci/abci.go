package abci

import (
	"abci-grpc/src/token"
	context "context"
	"flag"
	"log"
	"net"
	"net/http"

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
	log.Print("Received CheckTx()")

	message, err := token.DecodeMessage(in.Tx)

	if err != nil {
		log.Fatal("cannot decode Tx message")
	}

	err = s.token.ValidateMessage(message)

	if err != nil {
		log.Fatal(err)
	}

	return &EmptyMessage{}, nil
}

func (s *server) DeliverTx(ctx context.Context, in *DeliverTxRequest) (*EmptyMessage, error) {
	log.Print("Received DeliverTx()")

	message, err := token.DecodeMessage(in.Tx)

	if err != nil {
		log.Fatal("cannot decode Tx message")
	}

	err = s.token.ProcessMessage(message)

	if err != nil {
		log.Fatal(err)
	}

	return &EmptyMessage{}, nil
}

func (s *server) OnInitialize(ctx context.Context, in *BlockMessage) (*EmptyMessage, error) {
	log.Printf("Received OnInitialize(), block height: %d", in.Height)

	err := s.token.MineNewTokens(token.BASE_ACCOUNT)

	if err != nil {
		log.Fatal(err)
	}

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

func (s *server) GetAccountInfo(ctx context.Context, in *AccountMessage) (*AccountInfo, error) {
	log.Printf("GetAccountInfo(), account: %s", in.Account)

	amount, err := s.token.GetAccountInfo(in.Account)

	if err != nil {
		log.Fatal(err)
	}

	log.Printf("amount: %d", amount)

	return &AccountInfo{Amount: amount}, nil
}

func (s *server) CreateNewAccount(ctx context.Context, in *AccountMessage) (*AccountInfo, error) {
	log.Printf("CreateNewAccount(), account: %s", in.Account)

	err := s.token.CreateNewAccount(in.Account)

	if err != nil {
		log.Fatal(err)
	}
	return &AccountInfo{Amount: 0}, nil
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

	err = s.Serve(lis)

	if err != nil {
		glog.Fatal(err)
	}
}
