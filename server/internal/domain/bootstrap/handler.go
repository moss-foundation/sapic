package bootstrap

import (
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"

	"github.com/valyala/fasthttp"
)

type BootstrapRequest struct {
	Nonce        string `json:"nonce"`
	DevicePubKey string `json:"device_pubkey"`
}

type BootstrapResponse struct {
	DeviceToken string `json:"device_token"`
}

func BootstrapHandler(ctx *fasthttp.RequestCtx) {
	buildId := string(ctx.Request.Header.Peek("X-Build-Id"))
	timestamp := string(ctx.Request.Header.Peek("X-Timestamp"))
	signature := string(ctx.Request.Header.Peek("X-Bootstrap-Signature"))

	var req BootstrapRequest
	if err := json.Unmarshal(ctx.PostBody(), &req); err != nil {
		ctx.SetStatusCode(fasthttp.StatusBadRequest)
		ctx.SetBodyString("invalid JSON")
		return
	}

	if !verifyBootstrapSignature(signature, req, buildId, timestamp) {
		ctx.SetStatusCode(fasthttp.StatusUnauthorized)
		ctx.SetBodyString("invalid signature")
		return
	}

	token, err := generateRandomToken()
	if err != nil {
		ctx.SetStatusCode(fasthttp.StatusInternalServerError)
		ctx.SetBodyString("failed to generate token")
		return
	}

	resp, err := json.Marshal(BootstrapResponse{
		DeviceToken: *token,
	})
	if err != nil {
		ctx.SetStatusCode(fasthttp.StatusInternalServerError)
		ctx.SetBodyString("failed to marshal response")
		return
	}

	ctx.SetStatusCode(fasthttp.StatusOK)
	ctx.SetBody(resp)
}

const BOOTSTRAP_KEY = "super_secret_bootstrap_key"

func verifyBootstrapSignature(sig string, req BootstrapRequest, buildID, ts string) bool {
	mac := hmac.New(sha256.New, []byte(BOOTSTRAP_KEY))
	mac.Write([]byte(req.Nonce))
	mac.Write([]byte(req.DevicePubKey))
	mac.Write([]byte(buildID))
	mac.Write([]byte(ts))
	expected := hex.EncodeToString(mac.Sum(nil))
	return hmac.Equal([]byte(expected), []byte(sig))
}


func generateRandomToken() (*string, error) {
	b := make([]byte, 32)
	if _, err := rand.Read(b); err != nil {	
		return nil, err
	}

	token := hex.EncodeToString(b)
	return &token, nil
}