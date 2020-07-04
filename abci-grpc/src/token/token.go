package token

import (
	"encoding/json"
	"log"
	"strconv"

	"github.com/syndtr/goleveldb/leveldb"
)

const (
	BASE_TOKENS_AMOUNT = 50
	BASE_ACCOUNT       = "Alice"
)

type Token struct {
	db *leveldb.DB
}

type TokenMessage struct {
	From      string
	To        string
	Amount    uint64
	Signature string
}

type TokenError struct {
	err_message string
}

func (t_er *TokenError) Error() string {
	return t_er.err_message
}

func DecodeMessage(bytes []byte) (TokenMessage, error) {
	m := TokenMessage{}
	err := json.Unmarshal(bytes, &m)

	return m, err
}

func InitToken() Token {

	db, err := leveldb.OpenFile("token_data", nil)

	if err != nil {
		log.Fatal(err)
	}

	t := Token{db}

	t.CreateNewAccount(BASE_ACCOUNT)
	t.MineNewTokens(BASE_ACCOUNT)

	return t
}

func (t *Token) ValidateMessage(message TokenMessage) error {

	data, err := t.db.Get([]byte(message.From), nil)

	if err != nil {
		return &TokenError{"cant find account"}
	}

	available_amount, err := strconv.ParseUint(string(data), 10, 64)

	if available_amount < message.Amount {
		return &TokenError{"try to spend more than you have"}
	}

	return nil
}

func (t *Token) CreateNewAccount(account string) error {
	err := t.db.Put([]byte(account), []byte(strconv.FormatUint(0, 10)), nil)

	return err
}

func (t *Token) MineNewTokens(account string) error {
	data, err := t.db.Get([]byte(account), nil)

	if err != nil {
		return &TokenError{"cant find account"}
	}

	stored_amount, err := strconv.ParseUint(string(data), 10, 64)

	stored_amount += BASE_TOKENS_AMOUNT

	err = t.db.Put([]byte(account), []byte(strconv.FormatUint(stored_amount, 10)), nil)

	return err
}

func (t *Token) GetAccountInfo(account string) (uint64, error) {
	data, err := t.db.Get([]byte(account), nil)

	if err != nil {
		return 0, &TokenError{"cant find account"}
	}

	return strconv.ParseUint(string(data), 10, 64)
}

func (t *Token) ProcessMessage(message TokenMessage) error {

	data, err := t.db.Get([]byte(message.From), nil)

	if err != nil {
		return &TokenError{"cant find 'from' account"}
	}

	amount_from, err := strconv.ParseUint(string(data), 10, 64)

	if amount_from < message.Amount {
		return &TokenError{"try to spend more than you have"}
	}

	amount_from -= message.Amount

	data, err = t.db.Get([]byte(message.To), nil)

	if err != nil {
		return &TokenError{"cant find account 'to'"}
	}

	amount_to, err := strconv.ParseUint(string(data), 10, 64)

	amount_to += message.Amount

	err = t.db.Put([]byte(message.To), []byte(strconv.FormatUint(amount_to, 10)), nil)
	err = t.db.Put([]byte(message.From), []byte(strconv.FormatUint(amount_from, 10)), nil)

	return err
}

func (t *Token) StopToken() {
	t.db.Close()
}
