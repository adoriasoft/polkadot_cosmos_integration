package token

import (
	"crypto"
	"crypto/ecdsa"
	"crypto/elliptic"
	"log"
	"math/rand"
	"strconv"
	"strings"
)

const rand_min_len = 40

func GenerateKeyPair(private_key string) (*ecdsa.PublicKey, *ecdsa.PrivateKey) {
	var secp256k1 elliptic.Curve = elliptic.P256()

	for len(private_key) < 40+rand.Intn(100) {
		private_key += strconv.Itoa(rand.Intn(100))
	}

	rand := strings.NewReader(private_key)

	private, err := ecdsa.GenerateKey(secp256k1, rand)

	if err != nil {
		log.Fatal(err)
	}

	return &private.PublicKey, private
}

func Sign(message string, seed string, private_str string) (string, error) {
	private, err := SKBase64Decode(private_str)

	if err != nil {
		return "", err
	}

	var sha256_hasher = crypto.SHA256.New()

	m_hash := sha256_hasher.Sum([]byte(message))

	for len(seed) < 40 {
		seed += "0"
	}

	rand := strings.NewReader(seed)

	r, s, err := ecdsa.Sign(rand, private, m_hash)

	if err != nil {
		return "", err
	}

	signature := &Signature{r, s}

	return SGBase64Encode(signature), nil
}

func Verify(sign_str string, message string, public_str string) bool {
	sign, err := SGBase64Decode(sign_str)
	if err != nil {
		return false
	}

	public, err := PKBase64Decode(public_str)
	if err != nil {
		return false
	}

	var sha256_hasher = crypto.SHA256.New()

	m_hash := sha256_hasher.Sum([]byte(message))
	return ecdsa.Verify(public, m_hash, sign.r, sign.s)
}
