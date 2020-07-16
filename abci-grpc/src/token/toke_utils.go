package token

import (
	"log"
	"strconv"
)

func GenerateKeyPairForAccount(account_name string) (string, string) {

	public_key, private_key := GenerateKeyPair(account_name)

	public_key_encoded := PKBase64Encode(public_key)
	private_key_encoded := SKBase64Encode(private_key)

	log.Printf("%v, private key (save it!!!!): %v", account_name, private_key_encoded)

	log.Printf("Create new account message: { \"AccountName\": \"%v\", \"PublicKey\": \"%v\"}", account_name, public_key_encoded)

	return private_key_encoded, public_key_encoded
}

func GenerateTransactionMessage(from string, to string, amount int, private_key string, nonce int) {
	message := from + to + strconv.Itoa(amount) + strconv.Itoa(nonce)

	signature, err := Sign(message, "sign_seed", private_key)

	if err != nil {
		log.Fatal(err)
	}

	log.Printf("Transaction message: { \"From\": \"%v\", \"To\": \"%v\", \"Amount\": %v, \"Signature\": \"%v\", \"Nonce\": %v}", from, to, amount, signature, nonce)
}
