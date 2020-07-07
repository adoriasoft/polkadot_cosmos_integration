package token

import (
	"testing"
)

func TestBasicAccountFunctions(t *testing.T) {

	account_name := "Alice"

	token := InitToken("test_token")
	defer token.StopToken()

	_, err := token.CreateNewAccount(account_name)

	if err != nil {
		t.Error("cant create new account")
	}

	amount, err := token.GetAccountInfo(account_name)

	if err != nil {
		t.Error("cant get account amount")
	}

	if amount != 0 {
		t.Error("invalid")
	}

	err = token.MineNewTokens(account_name)

	if err != nil {
		t.Error("cant mine new tokens")
	}

	amount, err = token.GetAccountInfo(account_name)

	if err != nil {
		t.Error("cant get account amount")
	}

	if amount != BASE_TOKENS_AMOUNT {
		t.Error("invalid")
	}
}

func TestTokenMessages(t *testing.T) {

	alice_account := "Alice"
	bob_account := "Bob"

	token := InitToken("test_token")
	defer token.StopToken()

	alice_private_key, _ := token.CreateNewAccount(alice_account)
	token.CreateNewAccount(bob_account)

	token.MineNewTokens(alice_account)

	var message TokenMessage
	message.Amount = 2
	message.From = alice_account
	message.To = bob_account
	message.Message = "transaction from alice to bob"

	message.Signature, _ = Sign(message.Message, "seed1", alice_private_key)

	err := token.ValidateMessage(message)

	if err != nil {
		t.Error("invalid")
	}

	err = token.ProcessMessage(message)

	if err != nil {
		t.Error("invalid")
	}

	//corrupt message

	message.Signature = "corrupt"

	err = token.ProcessMessage(message)

	if err == nil {
		t.Error("invalid")
	}

}
