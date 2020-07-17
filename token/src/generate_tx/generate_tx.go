package main

import (
	"abci-grpc/src/token"
)

func main() {
	//alex_private_key := "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgaW9paLaNlodKXPe0zzxpqq+XHFQdP/1CM2EXTX9EROOhRANCAAShUsDxZfDaiMVvr3s4wQt801o8UaU0xH04Y57lojrTNsmDTZfQW9Ffiwgb2g3z8zoEtMsgQlfi5dxylqIpn0GG"

	alice_private_key := "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgnmlvaImLkJlJXvu7vQ7t4Y6rqH/5jVsyuTa6B5vGC7KhRANCAAQTCnjgKkLm/7X9lRF2R+04RubrNk4Z5i6nRQkBGWICHNmwgITyEI5I6NUNtHN71zrP0DPV8m6G7GYADX1O4WHw"

	token.GenerateTransactionMessage("Alice", "Alex", 1000, alice_private_key, 0)

	token.GenerateKeyPairForAccount("Alex")
}
