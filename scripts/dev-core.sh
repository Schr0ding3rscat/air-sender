#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../services/receiver-core"
AIR_SENDER_BIND="${AIR_SENDER_BIND:-127.0.0.1:9760}" \
AIR_SENDER_API_TOKEN="${AIR_SENDER_API_TOKEN:-dev-token}" \
cargo run
