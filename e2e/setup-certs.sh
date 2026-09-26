#!/usr/bin/env bash
# Generate the self-signed simulation CA + server cert (gitignored).
# SANs cover both mock hosts. No real credentials; local trust only.
set -euo pipefail

CERTS_DIR="$(dirname "$0")/.certs"
mkdir -p "$CERTS_DIR"

CA_KEY="$CERTS_DIR/ca-key.pem"
CA_PEM="$CERTS_DIR/ca.pem"
SRV_KEY="$CERTS_DIR/server-key.pem"
SRV_CSR="$CERTS_DIR/server.csr"
SRV_PEM="$CERTS_DIR/server.pem"

if [[ ! -f "$CA_PEM" ]]; then
  # CA extensions are mandatory: rustls/webpki rejects anchors without
  # basicConstraints CA:true (openssl/curl accept them — the app must not).
  openssl req -x509 -newkey rsa:2048 -nodes -days 7 \
    -subj "/CN=rusteams-sim-ca" \
    -addext "basicConstraints=critical,CA:true" \
    -addext "keyUsage=critical,keyCertSign,cRLSign,digitalSignature" \
    -keyout "$CA_KEY" -out "$CA_PEM" 2>/dev/null
fi

openssl req -newkey rsa:2048 -nodes \
  -subj "/CN=localhost" \
  -keyout "$SRV_KEY" -out "$SRV_CSR" 2>/dev/null

cat > "$CERTS_DIR/san.cnf" <<'EOF'
basicConstraints = CA:false
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = DNS:localhost, DNS:entra.mock.local, DNS:graph.mock.local, IP:127.0.0.1
EOF

openssl x509 -req -days 7 \
  -in "$SRV_CSR" -CA "$CA_PEM" -CAkey "$CA_KEY" -CAcreateserial \
  -extfile "$CERTS_DIR/san.cnf" \
  -out "$SRV_PEM" 2>/dev/null
rm -f "$SRV_CSR" "$CERTS_DIR/san.cnf" "$CERTS_DIR/ca.srl"
# Sim-only throwaway certs: the nginx container runs as a different user, so
# the mounted files must be world-readable. No real key material here.
chmod 644 "$CA_PEM" "$SRV_PEM" "$SRV_KEY"

echo "certs ready in $CERTS_DIR (ca.pem is RUSTEAMS_CA_BUNDLE)"
