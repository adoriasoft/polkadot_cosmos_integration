package main

import (
	"abci-grpc/src/token"
	"log"
	"strconv"
)

func main() {
	log.Print("Start sign")

	private_key_encoded := "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQglWBgX4mTlp1BVfS3uQfNPF9xfi7PeKxC4XLk1WU080GhRANCAARF09Qag4BNEEET4LYf3Q3w12k9AnoArBhB2cpZ1F3IqOZyFpbEOr63W2kbPz97p7OlFLAwqILHxsJvnHkqVQYy"

	message := "Alice" + "Bob" + strconv.Itoa(97)

	log.Print(message)

	signature, err := token.Sign(message, "seed2", private_key_encoded)

	if err != nil {
		log.Fatal(err)
	}

	log.Printf("signature: %v", signature)
}
