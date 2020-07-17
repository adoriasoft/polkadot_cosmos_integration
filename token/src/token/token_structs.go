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
	Amount    uint64 `json:"Amount"`
	Signature string `json:"Signature"`
	From      string `json:"From"`
	To        string `json:"To"`
	Nonce     uint64 `json:"Nonce"`
}

type NewAccountMessage struct {
	AccountName string `json:"AccountName"`
	PublicKey   string `json:"PublicKey"`
}

type TokenError struct {
	Err_message string
}

type Account struct {
	Amount    uint64 `default:"0"`
	PublicKey string
	Nonce     uint64 `default:"0"`
}

type Signature struct {
	r *big.Int
	s *big.Int
}

func (t_er *TokenError) Error() string {
	return t_er.Err_message
}

func DecodeMessage(bytes []byte) (TokenMessage, error) {
	var m TokenMessage
	err := json.Unmarshal(bytes, &m)

	if m.Amount == 0 && m.Nonce == 0 && m.From == "" && m.To == "" && m.Signature == "" {
		return m, &TokenError{"cant decode message"}
	}

	return m, err
}

func DecodeNewAccountMessage(bytes []byte) (NewAccountMessage, error) {
	var m NewAccountMessage
	err := json.Unmarshal(bytes, &m)

	if m.AccountName == "" && m.PublicKey == "" {
		return m, &TokenError{"cant decode message"}
	}

	return m, err
}

func DecodeAccountInfo(bytes []byte) (Account, error) {
	var a_info Account
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
