pub mod contracts;

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::Arc,
};

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProtocolKind {
    AirPlay,
    Cast,
    Miracast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCapabilities {
    pub max_resolution: String,
    pub max_fps: u16,
    pub audio: bool,
    pub recording: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDescriptor {
    pub id: String,
    pub kind: ProtocolKind,
    pub enabled: bool,
    pub capabilities: StreamCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SessionStatus {
    Pending,
    Active,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDescriptor {
    pub id: String,
    pub name: String,
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDescriptor {
    pub id: Uuid,
    pub protocol: ProtocolKind,
    pub device: DeviceDescriptor,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingProfile {
    pub destination_path: String,
    pub quality_preset: String,
    pub codec: String,
    pub container: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordingState {
    pub session_id: Uuid,
    pub profile: RecordingProfile,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub ts: DateTime<Utc>,
    pub kind: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AcceptancePolicy {
    Auto,
    Ask,
    TrustedOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiverPolicy {
    pub acceptance: AcceptancePolicy,
    pub max_sessions: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartRecordingRequest {
    pub session_id: Uuid,
    pub profile: RecordingProfile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopRecordingRequest {
    pub session_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSessionRequest {
    pub protocol: ProtocolKind,
    pub device_name: String,
    pub device_platform: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePolicyRequest {
    pub acceptance: Option<AcceptancePolicy>,
    pub max_sessions: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardSummary {
    pub protocol_count: usize,
    pub pending_sessions: usize,
    pub active_sessions: usize,
    pub stopped_sessions: usize,
    pub trusted_device_count: usize,
    pub active_recordings: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    api_token: String,
    protocols: Vec<ProtocolDescriptor>,
    sessions: Arc<RwLock<HashMap<Uuid, SessionDescriptor>>>,
    trusted_devices: Arc<RwLock<HashSet<String>>>,
    recordings: Arc<RwLock<HashMap<Uuid, RecordingState>>>,
    audit_log: Arc<RwLock<Vec<AuditEvent>>>,
    policy: Arc<RwLock<ReceiverPolicy>>,
}

impl AppState {
    pub fn bootstrap(api_token: String) -> Self {
        let protocols = vec![
            ProtocolDescriptor {
                id: "airplay".to_string(),
                kind: ProtocolKind::AirPlay,
                enabled: true,
                capabilities: StreamCapabilities {
                    max_resolution: "1920x1080".to_string(),
                    max_fps: 60,
                    audio: true,
                    recording: true,
                },
            },
            ProtocolDescriptor {
                id: "cast".to_string(),
                kind: ProtocolKind::Cast,
                enabled: true,
                capabilities: StreamCapabilities {
                    max_resolution: "1920x1080".to_string(),
                    max_fps: 60,
                    audio: true,
                    recording: true,
                },
            },
            ProtocolDescriptor {
                id: "miracast".to_string(),
                kind: ProtocolKind::Miracast,
                enabled: true,
                capabilities: StreamCapabilities {
                    max_resolution: "1920x1080".to_string(),
                    max_fps: 60,
                    audio: true,
                    recording: false,
                },
            },
        ];

        let mut sessions = HashMap::new();
        let seed_id = Uuid::new_v4();
        sessions.insert(
            seed_id,
            SessionDescriptor {
                id: seed_id,
                protocol: ProtocolKind::AirPlay,
                device: DeviceDescriptor {
                    id: "device-ios-seed".to_string(),
                    name: "Seed iPhone".to_string(),
                    platform: "iOS".to_string(),
                },
                status: SessionStatus::Pending,
                created_at: Utc::now(),
            },
        );

        Self {
            api_token,
            protocols,
            sessions: Arc::new(RwLock::new(sessions)),
            trusted_devices: Arc::new(RwLock::new(HashSet::new())),
            recordings: Arc::new(RwLock::new(HashMap::new())),
            audit_log: Arc::new(RwLock::new(Vec::new())),
            policy: Arc::new(RwLock::new(ReceiverPolicy {
                acceptance: AcceptancePolicy::Ask,
                max_sessions: 4,
            })),
        }
    }

    async fn audit(&self, kind: impl Into<String>, message: impl Into<String>) {
        let mut audit = self.audit_log.write().await;
        audit.push(AuditEvent {
            id: Uuid::new_v4(),
            ts: Utc::now(),
            kind: kind.into(),
            message: message.into(),
        });
    }

    fn authorize(&self, headers: &HeaderMap) -> Result<(), StatusCode> {
        let token = headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();

        if token == format!("Bearer {}", self.api_token) {
            Ok(())
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    fn unauthorized() -> (StatusCode, Json<ApiError>) {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                error: "unauthorized".into(),
            }),
        )
    }
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/dashboard", get(get_dashboard))
        .route("/v1/protocols", get(get_protocols))
        .route("/v1/sessions", get(get_sessions).post(create_mock_session))
        .route("/v1/sessions/:id/accept", post(accept_session))
        .route("/v1/sessions/:id/stop", post(stop_session))
        .route("/v1/recordings", get(get_recordings))
        .route("/v1/recordings/start", post(start_recording))
        .route("/v1/recordings/stop", post(stop_recording))
        .route("/v1/trust", get(get_trusted_devices))
        .route(
            "/v1/trust/:device_id",
            post(trust_device).delete(revoke_trust),
        )
        .route("/v1/audit", get(get_audit))
        .route("/v1/policy", get(get_policy).patch(update_policy))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn get_dashboard(State(state): State<AppState>) -> Json<DashboardSummary> {
    let sessions = state.sessions.read().await;
    let trusted = state.trusted_devices.read().await;
    let recordings = state.recordings.read().await;

    let mut pending = 0;
    let mut active = 0;
    let mut stopped = 0;

    for s in sessions.values() {
        match s.status {
            SessionStatus::Pending => pending += 1,
            SessionStatus::Active => active += 1,
            SessionStatus::Stopped => stopped += 1,
        }
    }

    Json(DashboardSummary {
        protocol_count: state.protocols.len(),
        pending_sessions: pending,
        active_sessions: active,
        stopped_sessions: stopped,
        trusted_device_count: trusted.len(),
        active_recordings: recordings.len(),
    })
}

async fn get_protocols(State(state): State<AppState>) -> Json<Vec<ProtocolDescriptor>> {
    Json(state.protocols.clone())
}

async fn get_sessions(State(state): State<AppState>) -> Json<Vec<SessionDescriptor>> {
    let sessions = state.sessions.read().await;
    Json(sessions.values().cloned().collect())
}

async fn create_mock_session(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    if let Err(_) = state.authorize(&headers) {
        state
            .audit("security.denied", "unauthorized create_session")
            .await;
        return AppState::unauthorized().into_response();
    }

    let policy = state.policy.read().await.clone();
    let sessions = state.sessions.read().await;
    let active_sessions = sessions
        .values()
        .filter(|s| s.status == SessionStatus::Active || s.status == SessionStatus::Pending)
        .count();
    drop(sessions);

    if active_sessions >= policy.max_sessions {
        return (
            StatusCode::CONFLICT,
            Json(ApiError {
                error: format!("max sessions reached ({})", policy.max_sessions),
            }),
        )
            .into_response();
    }

    let id = Uuid::new_v4();
    let descriptor = SessionDescriptor {
        id,
        protocol: payload.protocol,
        device: DeviceDescriptor {
            id: format!("device-{}", id.simple()),
            name: payload.device_name,
            platform: payload.device_platform,
        },
        status: SessionStatus::Pending,
        created_at: Utc::now(),
    };

    state
        .sessions
        .write()
        .await
        .insert(descriptor.id, descriptor.clone());
    state
        .audit(
            "session.created",
            format!("session {} created", descriptor.id),
        )
        .await;

    (StatusCode::CREATED, Json(descriptor)).into_response()
}

async fn accept_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Err(_) = state.authorize(&headers) {
        state
            .audit("security.denied", "unauthorized accept_session")
            .await;
        return AppState::unauthorized().into_response();
    }

    let mut sessions = state.sessions.write().await;
    match sessions.get_mut(&id) {
        Some(session) => {
            session.status = SessionStatus::Active;
            state
                .audit("session.accepted", format!("session {} accepted", id))
                .await;
            (StatusCode::OK, Json(session.clone())).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "session not found".into(),
            }),
        )
            .into_response(),
    }
}

async fn stop_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Err(_) = state.authorize(&headers) {
        state
            .audit("security.denied", "unauthorized stop_session")
            .await;
        return AppState::unauthorized().into_response();
    }

    let mut sessions = state.sessions.write().await;
    match sessions.get_mut(&id) {
        Some(session) => {
            session.status = SessionStatus::Stopped;
            state.recordings.write().await.remove(&id);
            state
                .audit("session.stopped", format!("session {} stopped", id))
                .await;
            (StatusCode::OK, Json(session.clone())).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "session not found".into(),
            }),
        )
            .into_response(),
    }
}

async fn get_recordings(State(state): State<AppState>) -> Json<Vec<RecordingState>> {
    Json(state.recordings.read().await.values().cloned().collect())
}

async fn start_recording(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<StartRecordingRequest>,
) -> impl IntoResponse {
    if let Err(_) = state.authorize(&headers) {
        state
            .audit("security.denied", "unauthorized start_recording")
            .await;
        return AppState::unauthorized().into_response();
    }

    let sessions = state.sessions.read().await;
    let Some(session) = sessions.get(&payload.session_id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "session not found".into(),
            }),
        )
            .into_response();
    };

    if session.status != SessionStatus::Active {
        return (
            StatusCode::CONFLICT,
            Json(ApiError {
                error: "session must be active to start recording".into(),
            }),
        )
            .into_response();
    }
    drop(sessions);

    let mut rec = state.recordings.write().await;
    rec.insert(
        payload.session_id,
        RecordingState {
            session_id: payload.session_id,
            profile: payload.profile,
            started_at: Utc::now(),
        },
    );
    state
        .audit(
            "recording.started",
            format!("recording started for {}", payload.session_id),
        )
        .await;

    (
        StatusCode::OK,
        Json(serde_json::json!({"session_id": payload.session_id, "status": "recording"})),
    )
        .into_response()
}

async fn stop_recording(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<StopRecordingRequest>,
) -> impl IntoResponse {
    if let Err(_) = state.authorize(&headers) {
        state
            .audit("security.denied", "unauthorized stop_recording")
            .await;
        return AppState::unauthorized().into_response();
    }

    let mut rec = state.recordings.write().await;
    if rec.remove(&payload.session_id).is_some() {
        state
            .audit(
                "recording.stopped",
                format!("recording stopped for {}", payload.session_id),
            )
            .await;
        (
            StatusCode::OK,
            Json(serde_json::json!({"session_id": payload.session_id, "status": "stopped"})),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "recording not active".into(),
            }),
        )
            .into_response()
    }
}

async fn get_trusted_devices(State(state): State<AppState>) -> Json<Vec<String>> {
    Json(state.trusted_devices.read().await.iter().cloned().collect())
}

async fn trust_device(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Err(_) = state.authorize(&headers) {
        state
            .audit("security.denied", "unauthorized trust_device")
            .await;
        return AppState::unauthorized().into_response();
    }

    state
        .trusted_devices
        .write()
        .await
        .insert(device_id.clone());
    state
        .audit("pairing.trusted", format!("trusted {}", device_id))
        .await;

    (
        StatusCode::OK,
        Json(serde_json::json!({ "trusted": device_id })),
    )
        .into_response()
}

async fn revoke_trust(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Err(_) = state.authorize(&headers) {
        state
            .audit("security.denied", "unauthorized revoke_trust")
            .await;
        return AppState::unauthorized().into_response();
    }

    if state.trusted_devices.write().await.remove(&device_id) {
        state
            .audit("pairing.revoked", format!("revoked {}", device_id))
            .await;
        (
            StatusCode::OK,
            Json(serde_json::json!({ "revoked": device_id })),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "device not trusted".into(),
            }),
        )
            .into_response()
    }
}

async fn get_audit(State(state): State<AppState>) -> Json<Vec<AuditEvent>> {
    Json(state.audit_log.read().await.clone())
}

async fn get_policy(State(state): State<AppState>) -> Json<ReceiverPolicy> {
    Json(state.policy.read().await.clone())
}

async fn update_policy(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdatePolicyRequest>,
) -> impl IntoResponse {
    if let Err(_) = state.authorize(&headers) {
        state
            .audit("security.denied", "unauthorized update_policy")
            .await;
        return AppState::unauthorized().into_response();
    }

    if let Some(max) = payload.max_sessions {
        if max == 0 || max > 4 {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "max_sessions must be between 1 and 4".into(),
                }),
            )
                .into_response();
        }
    }

    let mut policy = state.policy.write().await;
    if let Some(acceptance) = payload.acceptance {
        policy.acceptance = acceptance;
    }
    if let Some(max) = payload.max_sessions {
        policy.max_sessions = max;
    }

    state
        .audit("policy.updated", "receiver policy updated")
        .await;

    (StatusCode::OK, Json(policy.clone())).into_response()
}

pub async fn serve(addr: SocketAddr, api_token: String) {
    let state = AppState::bootstrap(api_token);
    let app = app(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind receiver-core listener");
    axum::serve(listener, app)
        .await
        .expect("serve receiver-core");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn seeded_session_id(app: &Router) -> Uuid {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/sessions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let sessions: Vec<SessionDescriptor> = serde_json::from_slice(&body).unwrap();
        sessions[0].id
    }

    #[tokio::test]
    async fn protocols_endpoint_works() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/protocols")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn mutating_endpoint_requires_auth() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/v1/trust/demo-device")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn trust_with_auth_succeeds() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/v1/trust/demo-device")
                    .header("authorization", "Bearer token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(String::from_utf8(body.to_vec())
            .unwrap()
            .contains("demo-device"));
    }

    #[tokio::test]
    async fn recording_requires_active_session() {
        let app = app(AppState::bootstrap("token".into()));
        let session_id = seeded_session_id(&app).await;
        let payload = serde_json::json!({
          "session_id": session_id,
          "profile": {
            "destination_path":"/tmp/out.mp4",
            "quality_preset":"balanced",
            "codec":"h264",
            "container":"mp4"
          }
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/v1/recordings/start")
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn update_policy_rejects_invalid_max_sessions() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PATCH")
                    .uri("/v1/policy")
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"max_sessions":5}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
