package token_crypt

import (
	"bytes"
	"crypto"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/x509"
	"encoding/base64"
	"encoding/binary"
	"log"
	"math/big"
	"strings"
)

const rand_min_len = 40

type Signature struct {
	r *big.Int
	s *big.Int
}

func SGBase64Encode(signature *Signature) string {
	buf := new(bytes.Buffer)

	r_bytes := signature.r.Bytes()
	s_bytes := signature.s.Bytes()

	binary.Write(buf, binary.LittleEndian, uint32(len(r_bytes)))
	binary.Write(buf, binary.LittleEndian, r_bytes)
	binary.Write(buf, binary.LittleEndian, uint32(len(s_bytes)))
	binary.Write(buf, binary.LittleEndian, s_bytes)

	return base64.StdEncoding.EncodeToString(buf.Bytes())
}

func SGBase64Decode(data string) (*Signature, error) {

	b, err := base64.StdEncoding.DecodeString(data)

	if err != nil {
		return nil, err
	}

	reader := bytes.NewReader(b)

	r := new(big.Int)
	s := new(big.Int)

	var r_size uint32
	binary.Read(reader, binary.LittleEndian, &r_size)
	r_bytes := make([]byte, r_size)
	binary.Read(reader, binary.LittleEndian, &r_bytes)
	r.SetBytes(r_bytes)

	var s_size uint32
	binary.Read(reader, binary.LittleEndian, &s_size)
	s_bytes := make([]byte, s_size)
	binary.Read(reader, binary.LittleEndian, &s_bytes)
	s.SetBytes(s_bytes)

	return &Signature{r: r, s: s}, nil
}

func PKBase64Encode(k *ecdsa.PrivateKey) string {
	bytes, err := x509.MarshalECPrivateKey(k)

	if err != nil {
		log.Fatal(err)
	}

	return base64.StdEncoding.EncodeToString(bytes)
}

func PKBase64Decode(data string) (*ecdsa.PrivateKey, error) {

	decoded, err := base64.StdEncoding.DecodeString(data)

	if err != nil {
		return nil, err
	}

	private_key, err := x509.ParseECPrivateKey(decoded)

	if err != nil {
		return nil, err
	}

	return private_key, nil
}

func GenerateKeyPair(private_key string) (*ecdsa.PublicKey, *ecdsa.PrivateKey) {
	var secp256k1 elliptic.Curve = elliptic.P256()

	for len(private_key) < 40 {
		private_key += "0"
	}

	rand := strings.NewReader(private_key)

	private, err := ecdsa.GenerateKey(secp256k1, rand)

	if err != nil {
		log.Fatal(err)
	}

	return &private.PublicKey, private
}

func Sign(message string, seed string, private *ecdsa.PrivateKey) *Signature {
	var sha256_hasher = crypto.SHA256.New()

	m_hash := sha256_hasher.Sum([]byte(message))

	for len(seed) < 40 {
		seed += "0"
	}

	rand := strings.NewReader(seed)

	r, s, err := ecdsa.Sign(rand, private, m_hash)

	if err != nil {
		log.Fatal(err)
	}

	return &Signature{r, s}
}

func Verify(sign *Signature, message string, public *ecdsa.PublicKey) bool {
	var sha256_hasher = crypto.SHA256.New()

	m_hash := sha256_hasher.Sum([]byte(message))
	return ecdsa.Verify(public, m_hash, sign.r, sign.s)
}
