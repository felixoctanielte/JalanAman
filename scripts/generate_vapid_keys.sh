#!/usr/bin/env bash
# Generates a VAPID key pair for Web Push notifications.
# Requires: openssl
set -euo pipefail

echo "Generating VAPID key pair..."

# Generate EC private key (P-256 curve)
openssl ecparam -genkey -name prime256v1 -noout -out vapid_private.pem

# Derive public key
openssl ec -in vapid_private.pem -pubout -out vapid_public.pem 2>/dev/null

# Export private key PEM (single line for .env)
PRIVATE_PEM=$(cat vapid_private.pem | tr '\n' '|' | sed 's/|/\\n/g')

# Export uncompressed public key as base64url (for browser push subscription)
PUBLIC_KEY_B64=$(openssl ec -in vapid_private.pem -pubout -outform DER 2>/dev/null \
  | tail -c 65 \
  | base64 \
  | tr '+/' '-_' \
  | tr -d '=\n')

echo ""
echo "══════════════════════════════════════════════════"
echo "Add these to your .env file:"
echo ""
echo "VAPID_PUBLIC_KEY=${PUBLIC_KEY_B64}"
echo "VAPID_PRIVATE_KEY_PEM=${PRIVATE_PEM}"
echo ""
echo "Private key saved to: vapid_private.pem (keep secret!)"
echo "Public key saved to:  vapid_public.pem"
echo "══════════════════════════════════════════════════"
