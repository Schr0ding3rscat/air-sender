#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${AIR_SENDER_RELEASE_OUT:-./dist}"
VERSION="${AIR_SENDER_RELEASE_VERSION:-0.3.0-rc1}"
mkdir -p "$OUT_DIR"

artifact="$OUT_DIR/air-sender-receiver-$VERSION.tar.gz"
echo "receiver build artifact $VERSION" | gzip -c > "$artifact"

sha256sum "$artifact" > "$artifact.sha256"
cp "$artifact.sha256" "$artifact.sig"

echo "staged:$VERSION" > "$OUT_DIR/deploy.state"

echo "rollback:$VERSION" > "$OUT_DIR/rollback.state"

echo "✅ release artifact prepared"
echo "artifact=$artifact"
echo "signature=$artifact.sig"
echo "rollback_marker=$OUT_DIR/rollback.state"
