#!/bin/bash
# Script to generate test certificates for C2PA signing
# WARNING: These certificates are for TESTING ONLY!
# Do not use in production.

set -e

CERT_DIR="examples/certs"
mkdir -p "$CERT_DIR"

echo "Generating test certificates for C2PA signing..."
echo "⚠️  WARNING: These are self-signed certificates for testing only!"
echo ""

# ES256 (Elliptic Curve)
echo "1. Generating ES256 certificate..."
openssl ecparam -name prime256v1 -genkey -noout -out "$CERT_DIR/es256_private.pem"
openssl req -new -x509 -key "$CERT_DIR/es256_private.pem" -out "$CERT_DIR/es256_cert.pem" -days 365 \
    -subj "/C=US/ST=Test/L=Test/O=Test Org/CN=C2PA Test ES256"
echo "   ✓ Created: $CERT_DIR/es256_private.pem and $CERT_DIR/es256_cert.pem"

# ES384 (Elliptic Curve)
echo "2. Generating ES384 certificate..."
openssl ecparam -name secp384r1 -genkey -noout -out "$CERT_DIR/es384_private.pem"
openssl req -new -x509 -key "$CERT_DIR/es384_private.pem" -out "$CERT_DIR/es384_cert.pem" -days 365 \
    -subj "/C=US/ST=Test/L=Test/O=Test Org/CN=C2PA Test ES384"
echo "   ✓ Created: $CERT_DIR/es384_private.pem and $CERT_DIR/es384_cert.pem"

# ES512 (Elliptic Curve)
echo "3. Generating ES512 certificate..."
openssl ecparam -name secp521r1 -genkey -noout -out "$CERT_DIR/es512_private.pem"
openssl req -new -x509 -key "$CERT_DIR/es512_private.pem" -out "$CERT_DIR/es512_cert.pem" -days 365 \
    -subj "/C=US/ST=Test/L=Test/O=Test Org/CN=C2PA Test ES512"
echo "   ✓ Created: $CERT_DIR/es512_private.pem and $CERT_DIR/es512_cert.pem"

# PS256 (RSA with PSS padding)
echo "4. Generating PS256 certificate..."
openssl genrsa -out "$CERT_DIR/ps256_private.pem" 2048
openssl req -new -x509 -key "$CERT_DIR/ps256_private.pem" -out "$CERT_DIR/ps256_cert.pem" -days 365 \
    -subj "/C=US/ST=Test/L=Test/O=Test Org/CN=C2PA Test PS256"
echo "   ✓ Created: $CERT_DIR/ps256_private.pem and $CERT_DIR/ps256_cert.pem"

echo ""
echo "✅ All test certificates generated successfully!"
echo ""
echo "Usage examples:"
echo ""
echo "# Using ES256 (recommended for testing):"
echo "./target/release/c2pa-testfile-maker \\"
echo "  --manifest examples/simple_manifest.json \\"
echo "  --input input.jpg \\"
echo "  --output output.jpg \\"
echo "  --cert $CERT_DIR/es256_cert.pem \\"
echo "  --key $CERT_DIR/es256_private.pem \\"
echo "  --algorithm es256"
echo ""
echo "# Using PS256:"
echo "./target/release/c2pa-testfile-maker \\"
echo "  --manifest examples/simple_manifest.json \\"
echo "  --input input.jpg \\"
echo "  --output output.jpg \\"
echo "  --cert $CERT_DIR/ps256_cert.pem \\"
echo "  --key $CERT_DIR/ps256_private.pem \\"
echo "  --algorithm ps256"
echo ""
echo "⚠️  Remember: These certificates are self-signed and intended for testing only!"
echo "   For production use, obtain certificates from a trusted Certificate Authority."

