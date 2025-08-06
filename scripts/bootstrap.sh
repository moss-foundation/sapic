#!/usr/bin/env bash

BOOTSTRAP_KEY="super_secret_bootstrap_key"
CLIENT_ID="myapp-desktop"
BUILD_ID="1.0.0"
NONCE=$(python3 -c "import os,binascii; print(binascii.hexlify(os.urandom(16)).decode())")
TIMESTAMP=$(date +%s)
DEVICE_PUBKEY="34472a914dc40a3fc0b493b86c6a14f3d10ef42e30b9eaab1c22a7a59e50dd2b"

DATA="${NONCE}${DEVICE_PUBKEY}${BUILD_ID}${TIMESTAMP}"
HMAC_SIG=$(printf '%s' "$DATA" | \
  openssl dgst -sha256 -hmac "$BOOTSTRAP_KEY" -hex | \
  sed 's/^.* //')

curl -X POST http://localhost:8080/api/v1/bootstrap \
  -H "Content-Type: application/json" \
  -H "X-Client-Id: $CLIENT_ID" \
  -H "X-Build-Id: $BUILD_ID" \
  -H "X-Timestamp: $TIMESTAMP" \
  -H "X-Bootstrap-Signature: $HMAC_SIG" \
  -d "{\"nonce\":\"$NONCE\",\"device_pubkey\":\"$DEVICE_PUBKEY\"}"
