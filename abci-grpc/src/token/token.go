package token

import (
	"log"

	"github.com/boltdb/bolt"
)

type Token struct {
	db *bolt.DB
}

func InitToken() Token {
	// Open the my.db data file in your current directory.
	// It will be created if it doesn't exist.
	db, err := bolt.Open("toke_data.db", 0600, nil)
	if err != nil {
		log.Fatal(err)
	}

	return Token{db}
}

func (t *Token) StopToken() {
	t.db.Close()
}
