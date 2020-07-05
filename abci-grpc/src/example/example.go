package main

import (
	"abci-grpc/src/token"
	"log"
)

func main() {
	log.Print("Start")

	public, private := token.GenerateKeyPair("private key")

	pub_encoded := token.PBKBase64Encode(public)

	public, _ = token.PBKBase64Decode(pub_encoded)

	encoded := token.PKBase64Encode(private)

	private, _ = token.PKBase64Decode(encoded)

	signature := token.Sign("sign this pls", "fucking seed", private)

	encoded_signature := token.SGBase64Encode(signature)
	signature, _ = token.SGBase64Decode(encoded_signature)

	result := token.Verify(signature, "sign this pls", public)

	log.Printf("Result: %t", result)
}
