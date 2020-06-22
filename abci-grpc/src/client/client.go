package main

import (
	"abci-server/src/abci"
	"context"
	"log"
	"os"
	"time"

	"google.golang.org/grpc"
)

const (
	address   = "localhost"
	port      = ":8081"
	defaultTx = "hello from client"
)

func main() {
	log.Printf("Start client")

	conn, err := grpc.Dial(address+port, grpc.WithInsecure(), grpc.WithBlock())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	log.Printf("connected...")

	defer conn.Close()

	client := abci.NewAbciClient(conn)

	tx := []byte(defaultTx)

	if len(os.Args) > 1 {
		tx = []byte(os.Args[1])
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	_, err = client.InitChain(ctx, &abci.EmptyMessage{})
	if err != nil {
		log.Fatalf("count not send")
	}

	_, err = client.CheckTx(ctx, &abci.CheckTxRequest{Tx: tx})
	if err != nil {
		log.Fatalf("count not send")
	}

	_, err = client.DeliverTx(ctx, &abci.DeliverTxRequest{Tx: tx})
	if err != nil {
		log.Fatalf("count not send")
	}

	_, err = client.OnInitialize(ctx, &abci.BlockMessage{Height: 15})
	if err != nil {
		log.Fatalf("count not send")
	}

	_, err = client.OnFinilize(ctx, &abci.BlockMessage{Height: 25})
	if err != nil {
		log.Fatalf("count not send")
	}

	_, err = client.Commit(ctx, &abci.BlockMessage{Height: 65})
	if err != nil {
		log.Fatalf("count not send")
	}

	log.Printf("Sucessfull sending")

}
