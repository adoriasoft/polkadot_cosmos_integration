package token

import (
	"os"
	"strconv"
	"testing"
)

func TestBasicAccountFunctions(t *testing.T) {

	account_name := "Alice"

	token := InitToken("test_token")
	defer token.StopToken()
	defer os.RemoveAll("test_token")

	_, public_key_encoded := GenerateKeyPairForAccount(account_name)

	err := token.CreateNewAccount(account_name, public_key_encoded)

	if err == nil {
		t.Errorf("Account Alice has not been created")
	}

	amount, _, err := token.GetAccountInfo(account_name)

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

	amount, _, err = token.GetAccountInfo(account_name)

	if err != nil {
		t.Error("cant get account amount")
	}

	if amount != BASE_TOKENS_AMOUNT {
		t.Error("invalid")
	}

	if err != nil {
		t.Error(err.Error())
	}

}

func TestTokenMessages(t *testing.T) {

	alex_account := "Alex"
	bob_account := "Bob"

	token := InitToken("test_token")
	defer token.StopToken()
	defer os.RemoveAll("test_token")

	alex_private_key, alex_public_key := GenerateKeyPairForAccount(alex_account)

	token.CreateNewAccount(alex_account, alex_public_key)

	token.MineNewTokens(alex_account)

	var message TokenMessage
	message.Amount = 10
	message.From = alex_account
	message.To = bob_account

	sign_message := message.From + message.To + strconv.FormatUint(message.Amount, 10) + strconv.FormatUint(message.Nonce, 10)

	message.Signature, _ = Sign(sign_message, "seed1", alex_private_key)

	err := token.ValidateMessage(message)

	if err != nil {
		t.Errorf("invalid: %v", err.Error())
	}

	err = token.ProcessMessage(message)

	if err != nil {
		t.Errorf("invalid: %v", err.Error())
	}

	//corrupt message

	message.Signature = "corrupt"

	err = token.ProcessMessage(message)

	if err == nil {
		t.Error("invalid")
	}

	// check bob amount

	bob_amount, _, err := token.GetAccountInfo(bob_account)

	if err != nil {
		t.Error("cant get account amount")
	}

	if bob_amount != 10 {
		t.Error("invalid amount")
	}

	bob_private_key, bob_public_key := GenerateKeyPairForAccount(bob_account)

	token.CreateNewAccount(bob_account, bob_public_key)

	message.From = "Bob"
	message.To = "Alex"
	message.Amount = 5

	sign_message = message.From + message.To + strconv.FormatUint(message.Amount, 10) + strconv.FormatUint(message.Nonce, 10)

	message.Signature, _ = Sign(sign_message, "seed1", bob_private_key)

	err = token.ProcessMessage(message)

	if err != nil {
		t.Errorf("invalid: %v", err.Error())
	}

	bob_amount, _, err = token.GetAccountInfo(bob_account)

	if bob_amount != 5 {
		t.Error("invalid amount")
	}

}

func TestMessageNonce(t *testing.T) {

	alex_account := "Alex"
	bob_account := "Bob"

	token := InitToken("test_token")
	defer token.StopToken()
	defer os.RemoveAll("test_token")

	alex_private_key, alex_public_key := GenerateKeyPairForAccount(alex_account)

	token.CreateNewAccount(alex_account, alex_public_key)

	token.MineNewTokens(alex_account)

	var message TokenMessage
	message.Amount = 10
	message.From = alex_account
	message.To = bob_account

	sign_message := message.From + message.To + strconv.FormatUint(message.Amount, 10) + strconv.FormatUint(message.Nonce, 10)

	message.Signature, _ = Sign(sign_message, "seed1", alex_private_key)

	err := token.ProcessMessage(message)

	if err != nil {
		t.Errorf("invalid: %v", err.Error())
	}

	// check bob amount

	bob_amount, _, _ := token.GetAccountInfo(bob_account)

	if bob_amount != 10 {
		t.Error("invalid amount")
	}

	err = token.ProcessMessage(message)

	if err == nil {
		t.Error("invalid nonce")
	}

	message.Nonce = 1

	sign_message = message.From + message.To + strconv.FormatUint(message.Amount, 10) + strconv.FormatUint(message.Nonce, 10)

	message.Signature, _ = Sign(sign_message, "seed1", alex_private_key)

	err = token.ProcessMessage(message)

	if err != nil {
		t.Errorf("invalid: %v", err.Error())
	}
}
