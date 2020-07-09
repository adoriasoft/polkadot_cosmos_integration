package token

import (
	context "context"
	"encoding/json"
	"log"
	"sync"

	"github.com/syndtr/goleveldb/leveldb"
)

const (
	BASE_TOKENS_AMOUNT = 50
	BASE_ACCOUNT       = "Alice"
)

type Token struct {
	db  *leveldb.DB
	mux sync.Mutex
}

func InitToken(db_name string) *Token {

	db, err := leveldb.OpenFile(db_name, nil)

	if err != nil {
		log.Fatal(err)
	}

	t := Token{db: db}

	t.CreateNewAccount(BASE_ACCOUNT)
	t.MineNewTokens(BASE_ACCOUNT)

	return &t
}

func (t *Token) ValidateMessage(message TokenMessage) error {
	t.mux.Lock()
	defer t.mux.Unlock()

	data, err := t.db.Get([]byte(message.From), nil)

	if err != nil {
		return &TokenError{"cant find account"}
	}

	var account_info AccountInfo
	json.Unmarshal(data, &account_info)

	if account_info.Amount < message.Amount {
		return &TokenError{"try to spend more than you have"}
	}

	if Verify(message.Signature, message.Message, account_info.PublicKey) == false {
		return &TokenError{"invalid signature"}
	}

	return nil
}

func (t *Token) CreateNewAccount(account string) (string, error) {
	t.mux.Lock()
	defer t.mux.Unlock()

	public_key, private_key := GenerateKeyPair(account)

	public_key_encoded := PBKBase64Encode(public_key)
	private_key_encoded := PKBase64Encode(private_key)

	account_info := AccountInfo{Amount: 0, PublicKey: public_key_encoded}

	bytes, _ := json.Marshal(account_info)

	err := t.db.Put([]byte(account), bytes, nil)

	return private_key_encoded, err
}

func (t *Token) MineNewTokens(account string) error {
	t.mux.Lock()
	defer t.mux.Unlock()

	data, err := t.db.Get([]byte(account), nil)

	if err != nil {
		return &TokenError{"cant find account"}
	}

	var account_info AccountInfo
	json.Unmarshal(data, &account_info)

	account_info.Amount += BASE_TOKENS_AMOUNT

	bytes, _ := json.Marshal(account_info)

	err = t.db.Put([]byte(account), bytes, nil)

	return err
}

func (t *Token) GetAccountInfo(account string) (uint64, error) {
	t.mux.Lock()
	defer t.mux.Unlock()

	data, err := t.db.Get([]byte(account), nil)

	if err != nil {
		return 0, &TokenError{"cant find account"}
	}

	var account_info AccountInfo
	json.Unmarshal(data, &account_info)

	return account_info.Amount, nil
}

func (t *Token) ProcessMessage(message TokenMessage) error {
	t.mux.Lock()
	defer t.mux.Unlock()

	data, err := t.db.Get([]byte(message.From), nil)

	if err != nil {
		return &TokenError{"cant find 'from' account"}
	}

	var account_from_info AccountInfo
	json.Unmarshal(data, &account_from_info)

	if account_from_info.Amount < message.Amount {
		return &TokenError{"try to spend more than you have"}
	}

	if Verify(message.Signature, message.Message, account_from_info.PublicKey) == false {
		return &TokenError{"invalid signature"}
	}

	account_from_info.Amount -= message.Amount

	data, err = t.db.Get([]byte(message.To), nil)

	if err != nil {
		return &TokenError{"cant find account 'to'"}
	}

	var account_to_info AccountInfo
	json.Unmarshal(data, &account_to_info)

	account_to_info.Amount += message.Amount

	bytes, _ := json.Marshal(account_to_info)
	t.db.Put([]byte(message.To), bytes, nil)

	bytes, _ = json.Marshal(account_from_info)
	t.db.Put([]byte(message.From), bytes, nil)

	return nil
}

func (t *Token) StopToken() {
	t.mux.Lock()
	defer t.mux.Unlock()

	t.db.Close()
}

type ServerToken struct {
	UnimplementedTokenServer
	Token *Token
}

func (s *ServerToken) CreateNewAccount(ctx context.Context, in *AccountMessage) (*AccountPrivateKey, error) {
	log.Printf("received CreateNewAccount(), account_name: %v", in.AccountName)

	private_key, err := s.Token.CreateNewAccount(in.AccountName)

	if err != nil {
		log.Fatal(err)
	}

	return &AccountPrivateKey{PrivateKey: private_key}, nil
}

func (s *ServerToken) GetBalance(ctx context.Context, in *AccountMessage) (*AccountBalance, error) {
	log.Printf("Received GetBalance()")

	balance, err := s.Token.GetAccountInfo(in.AccountName)

	if err != nil {
		log.Printf("Cannot find account")
	}

	log.Printf("account: %v, balance: %v", in.AccountName, balance)

	return &AccountBalance{Balance: balance}, nil
}
