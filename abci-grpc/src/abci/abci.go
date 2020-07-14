package abci

import (
	"abci-grpc/src/token"
	context "context"
	"log"
)

type ServerABCI struct {
	UnimplementedAbciServer
	Token *token.Token
}

func (s *ServerABCI) InitChain(ctx context.Context, in *EmptyMessage) (*EmptyMessage, error) {
	log.Print("received InitChain()")
	return &EmptyMessage{}, nil
}

func (s *ServerABCI) CheckTx(ctx context.Context, in *CheckTxRequest) (*EmptyMessage, error) {
	log.Print("Received CheckTx()")

	message, err := token.DecodeMessage(in.Tx)

	if err != nil {
		log.Print("cannot decode Tx message")
		return nil, err
	}

	err = s.Token.ValidateMessage(message)

	if err != nil {
		log.Print(err.Error())
		return nil, err
	}

	log.Print("Received CheckTx() successful")
	return &EmptyMessage{}, nil
}

func (s *ServerABCI) DeliverTx(ctx context.Context, in *DeliverTxRequest) (*EmptyMessage, error) {
	log.Print("Received DeliverTx()")

	message, err := token.DecodeMessage(in.Tx)

	if err != nil {
		log.Print("cannot decode Tx message")
		return nil, err
	}

	err = s.Token.ProcessMessage(message)

	if err != nil {
		log.Print(err.Error())
		return nil, err
	}

	log.Print("Received DeliverTx() successful")
	return &EmptyMessage{}, nil
}

func (s *ServerABCI) OnInitialize(ctx context.Context, in *BlockMessage) (*EmptyMessage, error) {
	log.Printf("Received OnInitialize(), block height: %d", in.Height)

	err := s.Token.MineNewTokens(token.BASE_ACCOUNT)

	if err != nil {
		log.Print("cannot mine new tokens")
		return nil, err
	}

	return &EmptyMessage{}, nil
}

func (s *ServerABCI) OnFinilize(ctx context.Context, in *BlockMessage) (*EmptyMessage, error) {
	log.Printf("Received OnFinilize(), block height: %d", in.Height)
	return &EmptyMessage{}, nil
}

func (s *ServerABCI) Commit(ctx context.Context, in *BlockMessage) (*EmptyMessage, error) {
	log.Printf("Received Commit(), block height: %d", in.Height)
	return &EmptyMessage{}, nil
}
