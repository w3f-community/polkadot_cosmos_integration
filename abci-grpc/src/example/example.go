package main

import (
	"abci-grpc/src/token"
	"log"
	"strconv"
)

func main() {
	log.Print("Start sign")

	//bob_private_key_encoded := "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgYGBgX72QndBBmdUlAJYi1Da/p3Njvybb/NJr7zMKkjGhRANCAASEUS2ykO7Fwf1U/Db6IzfWqgLIGjP1R/Uu4UAMEmWBar/26bUe7i0x6K8EIsAamPyu8pRpAIe9JmdKp2cAdZeb"

	alice_private_key_encoded := "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQglWBgX4mTlp1BVfS3uQfNPF9xfi7PeKxC4XLk1WU080GhRANCAARF09Qag4BNEEET4LYf3Q3w12k9AnoArBhB2cpZ1F3IqOZyFpbEOr63W2kbPz97p7OlFLAwqILHxsJvnHkqVQYy"

	message := "Alice" + "Bob" + strconv.Itoa(5000)

	log.Print(message)

	signature, err := token.Sign(message, "sign_seed", alice_private_key_encoded)

	if err != nil {
		log.Fatal(err)
	}

	log.Printf("signature: %v", signature)
}