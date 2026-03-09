#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${AIR_SENDER_BASE_URL:-http://127.0.0.1:9760}"
API_TOKEN="${AIR_SENDER_API_TOKEN:-dev-token}"

usage() {
  cat <<USAGE
Usage:
  $0 protocols
  $0 sessions
  $0 create-session <protocol> <device_name> <platform>
  $0 accept-session <session_id>
  $0 stop-session <session_id>
  $0 update-policy '<json-patch>'
  $0 export-audit
  $0 diagnostics
USAGE
}

call() {
  local method="$1" path="$2" body="${3:-}"
  if [[ -n "$body" ]]; then
    curl -sS -X "$method" "$BASE_URL$path" -H "Authorization: Bearer $API_TOKEN" -H "Content-Type: application/json" -d "$body"
  else
    curl -sS -X "$method" "$BASE_URL$path" -H "Authorization: Bearer $API_TOKEN" -H "Content-Type: application/json"
  fi
}

cmd="${1:-}"
case "$cmd" in
  protocols) curl -sS "$BASE_URL/v1/protocols" ;;
  sessions) curl -sS "$BASE_URL/v1/sessions" ;;
  create-session)
    [[ $# -eq 4 ]] || { usage; exit 1; }
    call POST /v1/sessions "{\"protocol\":\"$2\",\"device_name\":\"$3\",\"device_platform\":\"$4\"}" ;;
  accept-session) [[ $# -eq 2 ]] || { usage; exit 1; }; call POST "/v1/sessions/$2/accept" ;;
  stop-session) [[ $# -eq 2 ]] || { usage; exit 1; }; call POST "/v1/sessions/$2/stop" ;;
  update-policy) [[ $# -eq 2 ]] || { usage; exit 1; }; call PATCH /v1/policy "$2" ;;
  export-audit) curl -sS "$BASE_URL/v1/audit/export" ;;
  diagnostics) curl -sS "$BASE_URL/v1/diagnostics/bundle" ;;
  *) usage; exit 1 ;;
esac
