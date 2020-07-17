package token

import (
	context "context"
	"encoding/json"
	"log"
	"strconv"
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

func check_signature(message *TokenMessage, pk_encoded string) error {
	sign_message := message.From + message.To + strconv.FormatUint(message.Amount, 10) + strconv.FormatUint(message.Nonce, 10)

	if Verify(message.Signature, sign_message, pk_encoded) == false {
		return &TokenError{"invalid signature"}
	}

	return nil
}

func InitToken(db_name string) *Token {

	db, err := leveldb.OpenFile(db_name, nil)

	if err != nil {
		log.Fatal(err)
	}

	t := Token{db: db}

	private_key_encoded := "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgnmlvaImLkJlJXvu7vQ7t4Y6rqH/5jVsyuTa6B5vGC7KhRANCAAQTCnjgKkLm/7X9lRF2R+04RubrNk4Z5i6nRQkBGWICHNmwgITyEI5I6NUNtHN71zrP0DPV8m6G7GYADX1O4WHw"
	public_key_encoded := "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEEwp44CpC5v+1/ZURdkftOEbm6zZOGeYup0UJARliAhzZsICE8hCOSOjVDbRze9c6z9Az1fJuhuxmAA19TuFh8A=="

	log.Printf("%v private key: %v", BASE_ACCOUNT, private_key_encoded)
	log.Printf("%v public key: %v", BASE_ACCOUNT, public_key_encoded)

	t.CreateNewAccount(BASE_ACCOUNT, public_key_encoded)
	return &t
}

func (t *Token) ValidateMessage(message TokenMessage) error {
	t.mux.Lock()
	defer t.mux.Unlock()

	data, err := t.db.Get([]byte(message.From), nil)

	if err != nil {
		return &TokenError{"cant find account"}
	}

	var account_info Account
	json.Unmarshal(data, &account_info)

	if account_info.Amount < message.Amount {
		return &TokenError{"try to spend more than you have"}
	}

	if message.Nonce != account_info.Nonce {
		return &TokenError{"invalid nonce"}
	}

	return check_signature(&message, account_info.PublicKey)
}

func (t *Token) CreateNewAccount(account_name string, public_key_encoded string) error {
	t.mux.Lock()
	defer t.mux.Unlock()

	data, err := t.db.Get([]byte(account_name), nil)

	var account_info Account
	if err == nil {
		json.Unmarshal(data, &account_info)
		if account_info.PublicKey != "" {
			return &TokenError{"Account has been created"}
		}
	}

	_, err = PKBase64Decode(public_key_encoded)

	if err != nil {
		return &TokenError{"Invalid public key"}
	}

	account_info.PublicKey = public_key_encoded

	bytes, _ := json.Marshal(account_info)

	err = t.db.Put([]byte(account_name), bytes, nil)

	return err
}

func (t *Token) MineNewTokens(account string) error {
	t.mux.Lock()
	defer t.mux.Unlock()

	data, err := t.db.Get([]byte(account), nil)

	if err != nil {
		return &TokenError{"cant find account"}
	}

	var account_info Account
	json.Unmarshal(data, &account_info)

	account_info.Amount += BASE_TOKENS_AMOUNT

	bytes, _ := json.Marshal(account_info)

	err = t.db.Put([]byte(account), bytes, nil)

	return err
}

func (t *Token) GetAccountInfo(account string) (uint64, uint64, error) {
	t.mux.Lock()
	defer t.mux.Unlock()

	data, err := t.db.Get([]byte(account), nil)

	if err != nil {
		return 0, 0, &TokenError{"cant find account"}
	}

	var account_info Account
	json.Unmarshal(data, &account_info)

	return account_info.Amount, account_info.Nonce, nil
}

func (t *Token) ProcessMessage(message TokenMessage) error {
	t.mux.Lock()
	defer t.mux.Unlock()

	data, err := t.db.Get([]byte(message.From), nil)

	if err != nil {
		return &TokenError{"try to spend more than you have"}
	}

	var account_from_info Account
	json.Unmarshal(data, &account_from_info)

	if account_from_info.Amount < message.Amount {
		return &TokenError{"try to spend more than you have"}
	}

	if message.Nonce != account_from_info.Nonce {
		return &TokenError{"invalid nonce"}
	}

	err = check_signature(&message, account_from_info.PublicKey)

	if err != nil {
		return err
	}

	account_from_info.Amount -= message.Amount

	data, err = t.db.Get([]byte(message.To), nil)

	var account_to_info Account
	if err != nil {
		account_to_info.Amount = 0
		account_to_info.PublicKey = ""
	} else {
		json.Unmarshal(data, &account_to_info)
	}

	account_to_info.Amount += message.Amount
	account_from_info.Nonce++

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

func (s *ServerToken) GetAccountInfo(ctx context.Context, in *AccountMessage) (*AccountInfo, error) {
	log.Printf("Received GetAccountInfo()")

	balance, nonce, err := s.Token.GetAccountInfo(in.AccountName)

	if err != nil {
		log.Printf("Cannot find account")
	}

	log.Printf("account: %v, balance: %v, nonce: %v", in.AccountName, balance, nonce)

	return &AccountInfo{Balance: balance, Nonce: nonce}, nil
}
