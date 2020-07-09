package token

import (
	"bytes"
	"crypto/ecdsa"
	"crypto/x509"
	"encoding/base64"
	"encoding/binary"
	"encoding/json"
	"log"
	"math/big"
)

type TokenMessage struct {
	From      string
	To        string
	Amount    uint64
	Signature string
}

type TokenError struct {
	err_message string
}

type AccountInfo struct {
	Amount    uint64
	PublicKey string
}

type Signature struct {
	r *big.Int
	s *big.Int
}

func (t_er *TokenError) Error() string {
	return t_er.err_message
}

func DecodeMessage(bytes []byte) (TokenMessage, error) {
	var m TokenMessage
	err := json.Unmarshal(bytes, &m)

	return m, err
}

func DecodeAccountInfo(bytes []byte) (AccountInfo, error) {
	var a_info AccountInfo
	err := json.Unmarshal(bytes, &a_info)

	return a_info, err
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

func SKBase64Encode(k *ecdsa.PrivateKey) string {
	bytes, err := x509.MarshalPKCS8PrivateKey(k)

	if err != nil {
		log.Fatal(err)
	}

	return base64.StdEncoding.EncodeToString(bytes)
}

func SKBase64Decode(data string) (*ecdsa.PrivateKey, error) {

	decoded, err := base64.StdEncoding.DecodeString(data)

	if err != nil {
		return nil, err
	}

	private_key, err := x509.ParsePKCS8PrivateKey(decoded)

	if err != nil {
		return nil, err
	}

	return private_key.(*ecdsa.PrivateKey), nil
}

func PKBase64Encode(k *ecdsa.PublicKey) string {
	bytes, err := x509.MarshalPKIXPublicKey(k)

	if err != nil {
		log.Fatal(err)
	}

	return base64.StdEncoding.EncodeToString(bytes)

}

func PKBase64Decode(data string) (*ecdsa.PublicKey, error) {

	decoded, err := base64.StdEncoding.DecodeString(data)

	if err != nil {
		return nil, err
	}

	private_key, err := x509.ParsePKIXPublicKey(decoded)

	if err != nil {
		return nil, err
	}

	return private_key.(*ecdsa.PublicKey), nil
}
