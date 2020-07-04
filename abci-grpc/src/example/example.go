package main

import (
	"abci-grpc/src/token_crypt"
	"log"
)

func main() {
	log.Print("Start")

	public, private := token_crypt.GenerateKeyPair("private key")

	encoded := token_crypt.PKBase64Encode(private)

	private, _ = token_crypt.PKBase64Decode(encoded)

	signature := token_crypt.Sign("sign this pls", "fucking seed", private)

	encoded_signature := token_crypt.SGBase64Encode(signature)
	signature, _ = token_crypt.SGBase64Decode(encoded_signature)

	result := token_crypt.Verify(signature, "sign this pls", public)

	log.Printf("Result: %t", result)
}
