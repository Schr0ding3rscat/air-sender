#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${AIR_SENDER_TEST_BASE_URL:-http://127.0.0.1:9760}"
API_TOKEN="${AIR_SENDER_TEST_API_TOKEN:-dev-token}"
CYCLES="${AIR_SENDER_SOAK_CYCLES:-25}"

request() {
  local method="$1"; shift
  local path="$1"; shift
  local body="${1:-}"
  if [[ -n "$body" ]]; then
    curl -sS -X "$method" "$BASE_URL$path" -H "Authorization: Bearer $API_TOKEN" -H "Content-Type: application/json" -d "$body" -w $'\n%{http_code}'
  else
    curl -sS -X "$method" "$BASE_URL$path" -H "Authorization: Bearer $API_TOKEN" -H "Content-Type: application/json" -w $'\n%{http_code}'
  fi
}

json_body(){ echo "$1"|sed '$d'; }
status_code(){ echo "$1"|tail -n1; }

for i in $(seq 1 "$CYCLES"); do
  create_payload="{\"protocol\":\"air-play\",\"device_name\":\"Soak Sender $i\",\"device_platform\":\"iOS\"}"
  out="$(request POST /v1/sessions "$create_payload")"
  [[ "$(status_code "$out")" == "201" ]] || { echo "create failed cycle=$i"; exit 1; }
  sid="$(python3 -c 'import json,sys;print(json.loads(sys.argv[1])["id"])' "$(json_body "$out")")"

  out="$(request POST /v1/sessions/$sid/accept)"
  [[ "$(status_code "$out")" == "200" ]] || { echo "accept failed cycle=$i"; exit 1; }

  out="$(request POST /v1/sessions/$sid/stop)"
  [[ "$(status_code "$out")" == "200" ]] || { echo "stop failed cycle=$i"; exit 1; }

done

audit_count="$(curl -sS "$BASE_URL/v1/audit" | python3 -c 'import json,sys;print(len(json.load(sys.stdin)))')"
echo "✅ Soak complete cycles=$CYCLES audit_events=$audit_count"
