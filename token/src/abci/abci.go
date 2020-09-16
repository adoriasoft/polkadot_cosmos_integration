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
  
	return &proto.ResponseCheckTx{}, nil

	// tx_message, err1 := token.DecodeMessage(in.Tx)
	// _, err2 := token.DecodeNewAccountMessage(in.Tx)

	// if err1 != nil && err2 != nil {
	// 	log.Print("cannot decode Tx message")
	// 	return nil, &token.TokenError{"cannot decode Tx message"}
	// }

	// var error_message string
	// if err1 == nil {
	// 	err := s.Token.ValidateMessage(tx_message)

	// 	if err != nil {
	// 		error_message = err.Error()
	// 	} else {
	// 		log.Print("Received CheckTx() successful")
	// 		return &proto.ResponseCheckTx{}, nil
	// 	}
	// }

	// log.Print(error_message)
	// return nil, &token.TokenError{error_message}
}

func (s *ServerABCI) DeliverTx(ctx context.Context, in *DeliverTxRequest) (*EmptyMessage, error) {
	log.Print("Received DeliverTx()")
  
	return &proto.ResponseDeliverTx{}, nil

	// tx_message, err1 := token.DecodeMessage(in.Tx)
	// acc_message, err2 := token.DecodeNewAccountMessage(in.Tx)

	// if err1 != nil && err2 != nil {
	// 	log.Print("cannot decode Tx message")
	// 	return nil, &token.TokenError{"cannot decode Tx message"}
	// }

	// var error_message string
	// if err1 == nil {
	// 	err := s.Token.ProcessMessage(tx_message)

	// 	if err != nil {
	// 		error_message = err.Error()
	// 	} else {
	// 		log.Print("Received DeliverTx() successful")
	// 		return &proto.ResponseDeliverTx{}, nil
	// 	}
	// }

	// if err2 == nil {
	// 	err := s.Token.CreateNewAccount(acc_message.AccountName, acc_message.PublicKey)

	// 	if err != nil {
	// 		error_message = err.Error()
	// 	} else {
	// 		log.Print("Received DeliverTx() successful")
	// 		return &proto.ResponseDeliverTx{}, nil
	// 	}
	// }

	// log.Print(error_message)
	// return nil, &token.TokenError{error_message}
}

func (s *ServerABCI) OnInitialize(ctx context.Context, in *BlockMessage) (*EmptyMessage, error) {
	log.Printf("Received OnInitialize(), block height: %d", in.Height)

	// err := s.Token.MineNewTokens(token.BASE_ACCOUNT)

	// if err != nil {
	// 	log.Print("cannot mine new tokens")
	// 	return nil, err
	// }

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

func (s *ServerABCI) Echo(ctx context.Context, in *proto.RequestEcho) (*proto.ResponseEcho, error) {
	log.Printf("Received Echo()")
	log.Print(in.Message)
	return &proto.ResponseEcho{Message: in.Message}, nil
}
