#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
CORE_DIR="$ROOT_DIR/services/receiver-core"

BASE_URL="${AIR_SENDER_TEST_BASE_URL:-http://127.0.0.1:9760}"
API_TOKEN="${AIR_SENDER_TEST_API_TOKEN:-dev-token}"
RUN_CORE="${AIR_SENDER_TEST_RUN_CORE:-1}"
STARTUP_TIMEOUT_SEC="${AIR_SENDER_TEST_STARTUP_TIMEOUT_SEC:-60}"
PREBUILD_CORE="${AIR_SENDER_TEST_PREBUILD_CORE:-0}"

CORE_PID=""

cleanup() {
  if [[ -n "$CORE_PID" ]] && kill -0 "$CORE_PID" 2>/dev/null; then
    kill "$CORE_PID" 2>/dev/null || true
    wait "$CORE_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

request() {
  local method="$1"
  local path="$2"
  local auth="${3:-yes}"
  local body="${4:-}"

  local headers=(-H "Content-Type: application/json")
  if [[ "$auth" == "yes" ]]; then
    headers+=(-H "Authorization: Bearer $API_TOKEN")
  fi

  if [[ -n "$body" ]]; then
    curl -sS -X "$method" "$BASE_URL$path" "${headers[@]}" -d "$body" -w $'\n%{http_code}'
  else
    curl -sS -X "$method" "$BASE_URL$path" "${headers[@]}" -w $'\n%{http_code}'
  fi
}

expect_status() {
  local name="$1"
  local expected_status="$2"
  local output="$3"
  local status
  status="$(echo "$output" | tail -n1)"

  if [[ "$status" != "$expected_status" ]]; then
    echo "❌ $name expected HTTP $expected_status but got $status"
    echo "Response body:"
    echo "$output" | sed '$d'
    exit 1
  fi

  echo "✅ $name (HTTP $status)"
}

json_body() {
  echo "$1" | sed '$d'
}

json_get() {
  local payload="$1"
  local path="$2"
  python3 -c 'import json,sys
obj=json.loads(sys.argv[1])
for part in sys.argv[2].split("."):
    obj=obj[part]
print(obj)' "$payload" "$path"
}

if [[ "$RUN_CORE" == "1" ]]; then
  echo "Starting receiver-core locally..."

  if [[ "$PREBUILD_CORE" == "1" ]]; then
    echo "Prebuilding receiver-core..."
    (
      cd "$CORE_DIR"
      cargo build >/tmp/air-sender-core.log 2>&1
    )
  fi

  (
    cd "$CORE_DIR"
    AIR_SENDER_BIND="${BASE_URL#http://}" AIR_SENDER_API_TOKEN="$API_TOKEN" cargo run >/tmp/air-sender-core.log 2>&1
  ) &
  CORE_PID="$!"

  retries=$((STARTUP_TIMEOUT_SEC * 2))
  for _ in $(seq 1 "$retries"); do
    if curl -sS "$BASE_URL/health" >/dev/null 2>&1; then
      break
    fi
    sleep 0.5
  done
fi

health_status="$(curl -sS -o /dev/null -w '%{http_code}' "$BASE_URL/health")"
if [[ "$health_status" != "200" ]]; then
  echo "❌ receiver-core not reachable at $BASE_URL (health HTTP $health_status)"
  if [[ -f /tmp/air-sender-core.log ]]; then
    echo "Showing recent /tmp/air-sender-core.log output:"
    tail -n 120 /tmp/air-sender-core.log || true
  fi
  exit 1
fi

echo "Running full local API feature test against $BASE_URL"

for endpoint in /v1/dashboard /v1/protocols /v1/policy /v1/performance/report /v1/sessions /v1/recordings /v1/trust /v1/audit /v1/audit/export /v1/diagnostics/bundle /v1/operator/settings; do
  output="$(request GET "$endpoint" no)"
  expect_status "GET $endpoint" 200 "$output"
done

output="$(request POST "/v1/trust/local-device" no)"
expect_status "unauthorized mutating call" 401 "$output"

create_payload='{"protocol":"air-play","device_name":"Local Simulator","device_platform":"iOS","priority":"teacher","audio_mode":"full"}'
output="$(request POST "/v1/sessions" yes "$create_payload")"
expect_status "create simulated session" 201 "$output"
session_body="$(json_body "$output")"
SESSION_ID="$(json_get "$session_body" 'id')"
DEVICE_ID="$(json_get "$session_body" 'device.id')"

a_output="$(request POST "/v1/sessions/$SESSION_ID/accept" yes)"
expect_status "accept session" 200 "$a_output"


reconnect_payload='{"jitter_ms":120,"dropped":true}'
output="$(request POST "/v1/sessions/$SESSION_ID/reconnect" yes "$reconnect_payload")"
expect_status "reconnect active session" 200 "$output"

recording_payload="{\"session_id\":\"$SESSION_ID\",\"profile\":{\"destination_path\":\"/tmp/local-test.mp4\",\"quality_preset\":\"balanced\",\"codec\":\"h264\",\"container\":\"mp4\"}}"
output="$(request POST "/v1/recordings/start" yes "$recording_payload")"
expect_status "start recording" 200 "$output"

stop_recording_payload="{\"session_id\":\"$SESSION_ID\"}"
output="$(request POST "/v1/recordings/stop" yes "$stop_recording_payload")"
expect_status "stop recording" 200 "$output"

output="$(request PATCH "/v1/protocols/cast" yes '{"enabled":false}')"
expect_status "disable cast protocol" 200 "$output"
output="$(request PATCH "/v1/protocols/cast" yes '{"enabled":true}')"
expect_status "re-enable cast protocol" 200 "$output"

output="$(request POST "/v1/trust/$DEVICE_ID" yes)"
expect_status "trust created device" 200 "$output"
output="$(request DELETE "/v1/trust/$DEVICE_ID" yes)"
expect_status "revoke trusted device" 200 "$output"

policy_payload='{"acceptance":"ask","max_sessions":3,"queue_policy":"teacher-priority","audio_output_device":"hdmi-main","target_display":"display-2","scaling_mode":"fit","rotation_degrees":90,"preserve_aspect_ratio":true}'
output="$(request PATCH "/v1/policy" yes "$policy_payload")"
expect_status "update policy" 200 "$output"



output="$(request POST "/v1/pairing/pin" yes)"
expect_status "generate pairing pin" 200 "$output"

output="$(request PATCH "/v1/operator/settings" yes '{"device_name":"Lab Receiver","pin_policy":"first-pair-only","network_visibility":"private-only"}')"
expect_status "update operator settings" 200 "$output"

sign_payload='{"name":"nightly-profile","policy":{"acceptance":"ask","max_sessions":2,"queue_policy":"teacher-priority","audio_output_device":"hdmi-main","display":{"target_display":"display-2","scaling_mode":"fit","rotation_degrees":0,"preserve_aspect_ratio":true}},"operator":{"device_name":"Lab Receiver","pin_policy":"first-pair-only","network_visibility":"private-only"}}'
output="$(request POST "/v1/config-profiles/sign" yes "$sign_payload")"
expect_status "sign config profile" 200 "$output"
signed_body="$(json_body "$output")"
verify_payload="$(python3 -c 'import json,sys;obj=json.loads(sys.argv[1]);print(json.dumps({"profile":obj["profile"],"signature":obj["signature"]}))' "$signed_body")"
output="$(request POST "/v1/config-profiles/verify" yes "$verify_payload")"
expect_status "verify config profile" 200 "$output"

output="$(request POST "/v1/sessions/$SESSION_ID/stop" yes)"
expect_status "stop session" 200 "$output"

output="$(request GET "/v1/audit" no)"
expect_status "audit log still available" 200 "$output"

audit_body="$(json_body "$output")"
audit_count="$(python3 -c 'import json,sys; print(len(json.loads(sys.argv[1])))' "$audit_body")"
if [[ "$audit_count" -lt 6 ]]; then
  echo "❌ expected audit history to contain test actions, found only $audit_count events"
  exit 1
fi

echo "✅ Full local feature test completed successfully."
