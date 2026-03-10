pub mod contracts;

use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
};

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    Queued,
    Active,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum SessionPriority {
    Normal,
    Teacher,
    AdminOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum AudioMode {
    Full,
    AudioOnly,
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
    pub priority: SessionPriority,
    pub audio_mode: AudioMode,
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
#[serde(rename_all = "kebab-case")]
pub enum QueuePolicy {
    FirstIn,
    TeacherPriority,
    AdminOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScalingMode {
    Fit,
    Fill,
    ActualSize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayPolicy {
    pub target_display: String,
    pub scaling_mode: ScalingMode,
    pub rotation_degrees: u16,
    pub preserve_aspect_ratio: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiverPolicy {
    pub acceptance: AcceptancePolicy,
    pub max_sessions: usize,
    pub queue_policy: QueuePolicy,
    pub audio_output_device: String,
    pub display: DisplayPolicy,
    #[serde(default)]
    pub performance: PerformancePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePolicy {
    pub target_latency_ms: u16,
    pub max_bitrate_mbps: u16,
    pub baseline_profile: String,
    pub allow_4k_best_effort: bool,
}

impl Default for PerformancePolicy {
    fn default() -> Self {
        Self {
            target_latency_ms: 85,
            max_bitrate_mbps: 24,
            baseline_profile: "1080p60".to_string(),
            allow_4k_best_effort: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReliabilityPolicy {
    pub reconnect_grace_ms: u64,
    pub max_reconnect_attempts: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PinPolicy {
    Always,
    FirstPairOnly,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkVisibility {
    Lan,
    PrivateOnly,
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorSettings {
    pub device_name: String,
    pub pin_policy: PinPolicy,
    pub network_visibility: NetworkVisibility,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateOperatorSettingsRequest {
    pub device_name: Option<String>,
    pub pin_policy: Option<PinPolicy>,
    pub network_visibility: Option<NetworkVisibility>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PairingPinResponse {
    pub pin: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingPinState {
    pub enabled: bool,
    pub pin: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInstruction {
    pub protocol: String,
    pub hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectInstructionsResponse {
    pub receiver_name: String,
    pub local_url: String,
    pub pairing_pin: PairingPinState,
    pub protocol_hints: Vec<ProtocolInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedConfigProfile {
    pub name: String,
    pub issued_at: DateTime<Utc>,
    pub policy: ReceiverPolicy,
    pub operator: OperatorSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignConfigProfileRequest {
    pub name: String,
    pub policy: ReceiverPolicy,
    pub operator: OperatorSettings,
}

#[derive(Debug, Clone, Serialize)]
pub struct SignedConfigEnvelope {
    pub profile: SignedConfigProfile,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyConfigProfileRequest {
    pub profile: SignedConfigProfile,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifyConfigProfileResponse {
    pub valid: bool,
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
    pub priority: Option<SessionPriority>,
    pub audio_mode: Option<AudioMode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProtocolRequest {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePolicyRequest {
    pub acceptance: Option<AcceptancePolicy>,
    pub max_sessions: Option<usize>,
    pub queue_policy: Option<QueuePolicy>,
    pub audio_output_device: Option<String>,
    pub target_display: Option<String>,
    pub scaling_mode: Option<ScalingMode>,
    pub rotation_degrees: Option<u16>,
    pub preserve_aspect_ratio: Option<bool>,
    pub target_latency_ms: Option<u16>,
    pub max_bitrate_mbps: Option<u16>,
    pub baseline_profile: Option<String>,
    pub allow_4k_best_effort: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionReconnectRequest {
    pub jitter_ms: u16,
    pub dropped: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionReconnectResponse {
    pub session_id: Uuid,
    pub status: SessionStatus,
    pub reconnect_attempts: u8,
    pub resumed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ThroughputProbeResult {
    pub profile: String,
    pub expected_fps: u16,
    pub expected_latency_ms: u16,
    pub target_bitrate_mbps: u16,
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceReport {
    pub baseline_1080p60: ThroughputProbeResult,
    pub best_effort_4k: ThroughputProbeResult,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditExport {
    pub exported_at: DateTime<Utc>,
    pub format: String,
    pub total_events: usize,
    pub events: Vec<AuditEvent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticsBundle {
    pub generated_at: DateTime<Utc>,
    pub dashboard: DashboardSummary,
    pub policy: ReceiverPolicy,
    pub reliability: ReliabilityPolicy,
    pub sessions: Vec<SessionDescriptor>,
    pub active_recordings: Vec<RecordingState>,
    pub protocol_status: Vec<ProtocolDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PreviewStreamState {
    NoActiveStream,
    Connecting,
    Live,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewStateResponse {
    pub stream_state: PreviewStreamState,
    pub resolution: Option<String>,
    pub fps_target: Option<u16>,
    pub session_id: Option<Uuid>,
    pub protocol: Option<ProtocolKind>,
    pub device_name: Option<String>,
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
    bind_addr: SocketAddr,
    protocols: Arc<RwLock<Vec<ProtocolDescriptor>>>,
    sessions: Arc<RwLock<HashMap<Uuid, SessionDescriptor>>>,
    trusted_devices: Arc<RwLock<HashSet<String>>>,
    recordings: Arc<RwLock<HashMap<Uuid, RecordingState>>>,
    audit_log: Arc<RwLock<Vec<AuditEvent>>>,
    policy: Arc<RwLock<ReceiverPolicy>>,
    operator_settings: Arc<RwLock<OperatorSettings>>,
    pairing_pin: Arc<RwLock<Option<PairingPinResponse>>>,
    signing_secret: Arc<Vec<u8>>,
    reconnect_attempts: Arc<RwLock<HashMap<Uuid, u8>>>,
    reliability: Arc<RwLock<ReliabilityPolicy>>,
}

impl AppState {
    pub fn bootstrap(api_token: String) -> Self {
        Self::bootstrap_with_bind(
            api_token,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9760),
        )
    }

    pub fn bootstrap_with_bind(api_token: String, bind_addr: SocketAddr) -> Self {
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
                priority: SessionPriority::Normal,
                audio_mode: AudioMode::Full,
                status: SessionStatus::Pending,
                created_at: Utc::now(),
            },
        );

        Self {
            api_token: api_token.clone(),
            bind_addr,
            protocols: Arc::new(RwLock::new(protocols)),
            sessions: Arc::new(RwLock::new(sessions)),
            trusted_devices: Arc::new(RwLock::new(HashSet::new())),
            recordings: Arc::new(RwLock::new(HashMap::new())),
            audit_log: Arc::new(RwLock::new(Vec::new())),
            policy: Arc::new(RwLock::new(ReceiverPolicy {
                acceptance: AcceptancePolicy::Ask,
                max_sessions: 4,
                queue_policy: QueuePolicy::FirstIn,
                audio_output_device: "default-speaker".to_string(),
                display: DisplayPolicy {
                    target_display: "display-1".to_string(),
                    scaling_mode: ScalingMode::Fit,
                    rotation_degrees: 0,
                    preserve_aspect_ratio: true,
                },
                performance: PerformancePolicy::default(),
            })),
            operator_settings: Arc::new(RwLock::new(OperatorSettings {
                device_name: "Air Sender Receiver".to_string(),
                pin_policy: PinPolicy::Always,
                network_visibility: NetworkVisibility::Lan,
            })),
            pairing_pin: Arc::new(RwLock::new(None)),
            signing_secret: Arc::new(format!("{}-config-signing", api_token).into_bytes()),
            reconnect_attempts: Arc::new(RwLock::new(HashMap::new())),
            reliability: Arc::new(RwLock::new(ReliabilityPolicy {
                reconnect_grace_ms: 4_000,
                max_reconnect_attempts: 5,
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

    fn local_api_base_url(&self) -> String {
        let host = match self.bind_addr.ip() {
            IpAddr::V4(ipv4) if ipv4.is_unspecified() => Ipv4Addr::LOCALHOST.to_string(),
            IpAddr::V4(ipv4) => ipv4.to_string(),
            IpAddr::V6(ipv6) if ipv6.is_unspecified() => Ipv6Addr::LOCALHOST.to_string(),
            IpAddr::V6(ipv6) => ipv6.to_string(),
        };

        let formatted_host = if host.contains(':') {
            format!("[{host}]")
        } else {
            host
        };

        format!("http://{formatted_host}:{}", self.bind_addr.port())
    }
}

fn profile_signature(secret: &[u8], profile: &SignedConfigProfile) -> String {
    let payload = serde_json::to_vec(profile).expect("serialize profile for signing");
    let mut hash = 0xcbf29ce484222325u64;
    for b in secret.iter().chain(payload.iter()) {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/dashboard", get(get_dashboard))
        .route("/v1/protocols", get(get_protocols))
        .route("/v1/protocols/:id", patch(update_protocol))
        .route("/v1/sessions", get(get_sessions).post(create_mock_session))
        .route("/v1/sessions/:id/accept", post(accept_session))
        .route("/v1/sessions/:id/stop", post(stop_session))
        .route("/v1/sessions/:id/reconnect", post(reconnect_session))
        .route("/v1/recordings", get(get_recordings))
        .route("/v1/recordings/start", post(start_recording))
        .route("/v1/recordings/stop", post(stop_recording))
        .route("/v1/trust", get(get_trusted_devices))
        .route(
            "/v1/pairing/pin",
            get(get_pairing_pin_state).post(generate_pairing_pin),
        )
        .route("/v1/connect/instructions", get(get_connect_instructions))
        .route(
            "/v1/trust/:device_id",
            post(trust_device).delete(revoke_trust),
        )
        .route(
            "/v1/operator/settings",
            get(get_operator_settings).patch(update_operator_settings),
        )
        .route("/v1/audit", get(get_audit))
        .route("/v1/audit/export", get(export_audit))
        .route("/v1/policy", get(get_policy).patch(update_policy))
        .route("/v1/preview/state", get(get_preview_state))
        .route("/v1/performance/report", get(get_performance_report))
        .route("/v1/diagnostics/bundle", get(get_diagnostics_bundle))
        .route("/v1/config-profiles/sign", post(sign_config_profile))
        .route("/v1/config-profiles/verify", post(verify_config_profile))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn get_dashboard(State(state): State<AppState>) -> Json<DashboardSummary> {
    let sessions = state.sessions.read().await;
    let trusted = state.trusted_devices.read().await;
    let recordings = state.recordings.read().await;
    let protocols = state.protocols.read().await;

    let mut pending = 0;
    let mut active = 0;
    let mut stopped = 0;

    for s in sessions.values() {
        match s.status {
            SessionStatus::Pending | SessionStatus::Queued => pending += 1,
            SessionStatus::Active => active += 1,
            SessionStatus::Stopped => stopped += 1,
        }
    }

    Json(DashboardSummary {
        protocol_count: protocols.len(),
        pending_sessions: pending,
        active_sessions: active,
        stopped_sessions: stopped,
        trusted_device_count: trusted.len(),
        active_recordings: recordings.len(),
    })
}

async fn get_protocols(State(state): State<AppState>) -> Json<Vec<ProtocolDescriptor>> {
    Json(state.protocols.read().await.clone())
}

async fn update_protocol(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProtocolRequest>,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
        state
            .audit("security.denied", "unauthorized update_protocol")
            .await;
        return AppState::unauthorized().into_response();
    }

    let mut protocols = state.protocols.write().await;
    let Some(protocol) = protocols.iter_mut().find(|p| p.id == id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "protocol not found".into(),
            }),
        )
            .into_response();
    };

    protocol.enabled = payload.enabled;
    state
        .audit(
            "protocol.updated",
            format!("protocol {} enabled={}", protocol.id, protocol.enabled),
        )
        .await;

    (StatusCode::OK, Json(protocol.clone())).into_response()
}

async fn get_sessions(State(state): State<AppState>) -> Json<Vec<SessionDescriptor>> {
    let sessions = state.sessions.read().await;
    Json(sessions.values().cloned().collect())
}

async fn get_preview_state(State(state): State<AppState>) -> Json<PreviewStateResponse> {
    let sessions = state.sessions.read().await;
    let protocols = state.protocols.read().await;

    let active = sessions
        .values()
        .find(|session| session.status == SessionStatus::Active)
        .cloned();
    let pending = sessions
        .values()
        .find(|session| {
            matches!(
                session.status,
                SessionStatus::Pending | SessionStatus::Queued
            )
        })
        .cloned();

    let (stream_state, current_session) = if let Some(session) = active {
        (PreviewStreamState::Live, Some(session))
    } else if let Some(session) = pending {
        (PreviewStreamState::Connecting, Some(session))
    } else {
        (PreviewStreamState::NoActiveStream, None)
    };

    let protocol_meta = current_session.as_ref().and_then(|session| {
        protocols
            .iter()
            .find(|protocol| protocol.kind == session.protocol)
    });

    Json(PreviewStateResponse {
        stream_state,
        resolution: protocol_meta.map(|protocol| protocol.capabilities.max_resolution.clone()),
        fps_target: protocol_meta.map(|protocol| protocol.capabilities.max_fps),
        session_id: current_session.as_ref().map(|session| session.id),
        protocol: current_session
            .as_ref()
            .map(|session| session.protocol.clone()),
        device_name: current_session.map(|session| session.device.name),
    })
}

async fn create_mock_session(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
        state
            .audit("security.denied", "unauthorized create_session")
            .await;
        return AppState::unauthorized().into_response();
    }

    let protocols = state.protocols.read().await;
    if !protocols
        .iter()
        .any(|p| p.kind == payload.protocol && p.enabled)
    {
        return (
            StatusCode::CONFLICT,
            Json(ApiError {
                error: "protocol is disabled".into(),
            }),
        )
            .into_response();
    }
    drop(protocols);

    let policy = state.policy.read().await.clone();
    let sessions = state.sessions.read().await;
    let active_sessions = sessions
        .values()
        .filter(|s| s.status == SessionStatus::Active || s.status == SessionStatus::Pending)
        .count();
    drop(sessions);

    let id = Uuid::new_v4();
    let priority = payload.priority.unwrap_or(SessionPriority::Normal);
    let audio_mode = payload.audio_mode.unwrap_or(AudioMode::Full);

    if matches!(audio_mode, AudioMode::AudioOnly)
        && !matches!(payload.protocol, ProtocolKind::AirPlay | ProtocolKind::Cast)
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "audio-only mode is only supported for airplay/cast".into(),
            }),
        )
            .into_response();
    }

    let status = if active_sessions >= policy.max_sessions {
        SessionStatus::Queued
    } else {
        match policy.acceptance {
            AcceptancePolicy::Auto => SessionStatus::Active,
            AcceptancePolicy::Ask => SessionStatus::Pending,
            AcceptancePolicy::TrustedOnly => SessionStatus::Pending,
        }
    };

    let descriptor = SessionDescriptor {
        id,
        protocol: payload.protocol,
        device: DeviceDescriptor {
            id: format!("device-{}", id.simple()),
            name: payload.device_name,
            platform: payload.device_platform,
        },
        priority,
        audio_mode,
        status,
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

    if descriptor.status == SessionStatus::Queued {
        state
            .audit(
                "session.queued",
                format!("session {} queued due to policy limit", descriptor.id),
            )
            .await;
    }

    (StatusCode::CREATED, Json(descriptor)).into_response()
}

async fn accept_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
        state
            .audit("security.denied", "unauthorized accept_session")
            .await;
        return AppState::unauthorized().into_response();
    }

    let mut sessions = state.sessions.write().await;
    let Some(target_session) = sessions.get(&id).cloned() else {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "session not found".into(),
            }),
        )
            .into_response();
    };

    if target_session.status == SessionStatus::Stopped {
        return (
            StatusCode::CONFLICT,
            Json(ApiError {
                error: "cannot accept a stopped session".into(),
            }),
        )
            .into_response();
    }

    let policy = state.policy.read().await.clone();
    let active_count = sessions
        .values()
        .filter(|s| s.status == SessionStatus::Active)
        .count();

    if active_count >= policy.max_sessions && target_session.status != SessionStatus::Active {
        let maybe_handoff_id = sessions
            .values()
            .filter(|s| s.status == SessionStatus::Active)
            .min_by_key(|s| {
                let weight = match policy.queue_policy {
                    QueuePolicy::FirstIn => 0,
                    QueuePolicy::TeacherPriority => match s.priority {
                        SessionPriority::AdminOverride => 2,
                        SessionPriority::Teacher => 1,
                        SessionPriority::Normal => 0,
                    },
                    QueuePolicy::AdminOverride => match s.priority {
                        SessionPriority::AdminOverride => 3,
                        SessionPriority::Teacher => 2,
                        SessionPriority::Normal => 1,
                    },
                };
                (weight, s.created_at)
            })
            .map(|s| s.id);

        if let Some(handoff_id) = maybe_handoff_id {
            if let Some(active_session) = sessions.get_mut(&handoff_id) {
                active_session.status = SessionStatus::Stopped;
                state.recordings.write().await.remove(&handoff_id);
                state
                    .audit(
                        "session.handoff",
                        format!("session {} stopped for handoff", handoff_id),
                    )
                    .await;
            }
        }
    }

    let session = sessions
        .get_mut(&id)
        .expect("session existence validated above");
    session.status = SessionStatus::Active;
    state
        .audit("session.accepted", format!("session {} accepted", id))
        .await;
    (StatusCode::OK, Json(session.clone())).into_response()
}

async fn stop_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
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

async fn reconnect_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<SessionReconnectRequest>,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
        state
            .audit("security.denied", "unauthorized reconnect_session")
            .await;
        return AppState::unauthorized().into_response();
    }

    let reliability = state.reliability.read().await.clone();
    let mut sessions = state.sessions.write().await;
    let Some(session) = sessions.get_mut(&id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "session not found".into(),
            }),
        )
            .into_response();
    };

    if session.status == SessionStatus::Stopped {
        return (
            StatusCode::CONFLICT,
            Json(ApiError {
                error: "cannot reconnect a stopped session".into(),
            }),
        )
            .into_response();
    }

    let mut attempts = state.reconnect_attempts.write().await;
    let count = attempts.entry(id).or_insert(0);
    *count = count.saturating_add(1);

    let resumed = payload.dropped
        && u64::from(payload.jitter_ms) <= reliability.reconnect_grace_ms
        && *count <= reliability.max_reconnect_attempts;

    session.status = if resumed {
        SessionStatus::Active
    } else {
        SessionStatus::Queued
    };

    state
        .audit(
            "session.reconnect",
            format!(
                "session {} reconnect dropped={} jitter_ms={} resumed={} attempts={}",
                id, payload.dropped, payload.jitter_ms, resumed, *count
            ),
        )
        .await;

    (
        StatusCode::OK,
        Json(SessionReconnectResponse {
            session_id: id,
            status: session.status.clone(),
            reconnect_attempts: *count,
            resumed,
        }),
    )
        .into_response()
}

async fn get_recordings(State(state): State<AppState>) -> Json<Vec<RecordingState>> {
    Json(state.recordings.read().await.values().cloned().collect())
}

async fn start_recording(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<StartRecordingRequest>,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
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
    if state.authorize(&headers).is_err() {
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

async fn get_pairing_pin_state(State(state): State<AppState>) -> Json<PairingPinState> {
    let settings = state.operator_settings.read().await.clone();
    let pin = state.pairing_pin.read().await.clone();

    let enabled = settings.pin_policy != PinPolicy::Disabled;

    let active_pin = pin.filter(|candidate| candidate.expires_at > Utc::now());
    Json(PairingPinState {
        enabled,
        pin: active_pin.as_ref().map(|candidate| candidate.pin.clone()),
        expires_at: active_pin.as_ref().map(|candidate| candidate.expires_at),
    })
}

async fn get_connect_instructions(
    State(state): State<AppState>,
) -> Json<ConnectInstructionsResponse> {
    let operator_settings = state.operator_settings.read().await.clone();
    let pairing_pin = get_pairing_pin_state(State(state.clone())).await.0;

    Json(ConnectInstructionsResponse {
        receiver_name: operator_settings.device_name,
        local_url: format!("{}/v1/connect/instructions", state.local_api_base_url()),
        pairing_pin,
        protocol_hints: vec![
            ProtocolInstruction {
                protocol: "AirPlay".to_string(),
                hint: "On iPhone, open Control Center, tap Screen Mirroring, then choose this receiver.".to_string(),
            },
            ProtocolInstruction {
                protocol: "Cast".to_string(),
                hint: "In Chrome or Android apps, tap Cast and pick this receiver name.".to_string(),
            },
            ProtocolInstruction {
                protocol: "Miracast".to_string(),
                hint: "On Windows, press Win+K and select this receiver from wireless displays.".to_string(),
            },
        ],
    })
}

async fn generate_pairing_pin(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
        state
            .audit("security.denied", "unauthorized generate_pairing_pin")
            .await;
        return AppState::unauthorized().into_response();
    }

    let pin = format!("{:06}", Utc::now().timestamp_subsec_millis() % 1_000_000);
    let response = PairingPinResponse {
        pin,
        expires_at: Utc::now() + chrono::Duration::minutes(5),
    };

    state.pairing_pin.write().await.replace(response.clone());

    state
        .audit("pairing.pin.generated", "on-screen pin generated")
        .await;
    (StatusCode::OK, Json(response)).into_response()
}

async fn trust_device(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
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
    if state.authorize(&headers).is_err() {
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

async fn export_audit(State(state): State<AppState>) -> Json<AuditExport> {
    let events = state.audit_log.read().await.clone();
    Json(AuditExport {
        exported_at: Utc::now(),
        format: "jsonl-compatible".to_string(),
        total_events: events.len(),
        events,
    })
}

async fn get_operator_settings(State(state): State<AppState>) -> Json<OperatorSettings> {
    Json(state.operator_settings.read().await.clone())
}

async fn update_operator_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateOperatorSettingsRequest>,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
        state
            .audit("security.denied", "unauthorized update_operator_settings")
            .await;
        return AppState::unauthorized().into_response();
    }

    let mut settings = state.operator_settings.write().await;
    if let Some(device_name) = payload.device_name {
        if device_name.trim().is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "device_name cannot be empty".into(),
                }),
            )
                .into_response();
        }
        settings.device_name = device_name;
    }
    if let Some(pin_policy) = payload.pin_policy {
        settings.pin_policy = pin_policy;
    }
    if let Some(network_visibility) = payload.network_visibility {
        settings.network_visibility = network_visibility;
    }

    state
        .audit("operator.updated", "operator settings updated")
        .await;
    (StatusCode::OK, Json(settings.clone())).into_response()
}

async fn get_policy(State(state): State<AppState>) -> Json<ReceiverPolicy> {
    Json(state.policy.read().await.clone())
}

async fn update_policy(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdatePolicyRequest>,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
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

    if let Some(rotation_degrees) = payload.rotation_degrees {
        if !matches!(rotation_degrees, 0 | 90 | 180 | 270) {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "rotation_degrees must be one of 0, 90, 180, 270".into(),
                }),
            )
                .into_response();
        }
    }

    if let Some(target_latency_ms) = payload.target_latency_ms {
        if !(30..=300).contains(&target_latency_ms) {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "target_latency_ms must be between 30 and 300".into(),
                }),
            )
                .into_response();
        }
    }

    if let Some(max_bitrate_mbps) = payload.max_bitrate_mbps {
        if !(8..=120).contains(&max_bitrate_mbps) {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "max_bitrate_mbps must be between 8 and 120".into(),
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
    if let Some(queue_policy) = payload.queue_policy {
        policy.queue_policy = queue_policy;
    }
    if let Some(audio_output_device) = payload.audio_output_device {
        if audio_output_device.trim().is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "audio_output_device cannot be empty".into(),
                }),
            )
                .into_response();
        }
        policy.audio_output_device = audio_output_device;
    }
    if let Some(target_display) = payload.target_display {
        if target_display.trim().is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "target_display cannot be empty".into(),
                }),
            )
                .into_response();
        }
        policy.display.target_display = target_display;
    }
    if let Some(scaling_mode) = payload.scaling_mode {
        policy.display.scaling_mode = scaling_mode;
    }
    if let Some(rotation_degrees) = payload.rotation_degrees {
        policy.display.rotation_degrees = rotation_degrees;
    }
    if let Some(preserve_aspect_ratio) = payload.preserve_aspect_ratio {
        policy.display.preserve_aspect_ratio = preserve_aspect_ratio;
    }
    if let Some(target_latency_ms) = payload.target_latency_ms {
        policy.performance.target_latency_ms = target_latency_ms;
    }
    if let Some(max_bitrate_mbps) = payload.max_bitrate_mbps {
        policy.performance.max_bitrate_mbps = max_bitrate_mbps;
    }
    if let Some(baseline_profile) = payload.baseline_profile {
        if baseline_profile.trim().is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "baseline_profile cannot be empty".into(),
                }),
            )
                .into_response();
        }
        policy.performance.baseline_profile = baseline_profile;
    }
    if let Some(allow_4k_best_effort) = payload.allow_4k_best_effort {
        policy.performance.allow_4k_best_effort = allow_4k_best_effort;
    }

    state
        .audit("policy.updated", "receiver policy updated")
        .await;

    (StatusCode::OK, Json(policy.clone())).into_response()
}

async fn get_performance_report(State(state): State<AppState>) -> Json<PerformanceReport> {
    let policy = state.policy.read().await.clone();
    let baseline = ThroughputProbeResult {
        profile: policy.performance.baseline_profile,
        expected_fps: 60,
        expected_latency_ms: policy.performance.target_latency_ms,
        target_bitrate_mbps: policy.performance.max_bitrate_mbps,
    };

    let best_effort_4k = ThroughputProbeResult {
        profile: if policy.performance.allow_4k_best_effort {
            "4k-best-effort".to_string()
        } else {
            "disabled".to_string()
        },
        expected_fps: if policy.performance.allow_4k_best_effort {
            45
        } else {
            0
        },
        expected_latency_ms: policy.performance.target_latency_ms.saturating_add(35),
        target_bitrate_mbps: policy.performance.max_bitrate_mbps.saturating_add(12),
    };

    Json(PerformanceReport {
        baseline_1080p60: baseline,
        best_effort_4k,
    })
}

async fn get_diagnostics_bundle(State(state): State<AppState>) -> Json<DiagnosticsBundle> {
    let sessions: Vec<SessionDescriptor> = state.sessions.read().await.values().cloned().collect();
    let recordings: Vec<RecordingState> = state.recordings.read().await.values().cloned().collect();
    let protocol_status = state.protocols.read().await.clone();
    let policy = state.policy.read().await.clone();
    let reliability = state.reliability.read().await.clone();
    let dashboard = get_dashboard(State(state.clone())).await.0;

    Json(DiagnosticsBundle {
        generated_at: Utc::now(),
        dashboard,
        policy,
        reliability,
        sessions,
        active_recordings: recordings,
        protocol_status,
    })
}

async fn sign_config_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SignConfigProfileRequest>,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
        state
            .audit("security.denied", "unauthorized sign_config_profile")
            .await;
        return AppState::unauthorized().into_response();
    }

    if payload.name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "profile name cannot be empty".into(),
            }),
        )
            .into_response();
    }

    let profile = SignedConfigProfile {
        name: payload.name,
        issued_at: Utc::now(),
        policy: payload.policy,
        operator: payload.operator,
    };
    let signature = profile_signature(&state.signing_secret, &profile);
    state
        .audit("config.signed", "signed configuration profile")
        .await;

    (
        StatusCode::OK,
        Json(SignedConfigEnvelope { profile, signature }),
    )
        .into_response()
}

async fn verify_config_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<VerifyConfigProfileRequest>,
) -> impl IntoResponse {
    if state.authorize(&headers).is_err() {
        state
            .audit("security.denied", "unauthorized verify_config_profile")
            .await;
        return AppState::unauthorized().into_response();
    }

    let expected = profile_signature(&state.signing_secret, &payload.profile);
    let valid = expected == payload.signature;
    state
        .audit(
            "config.verified",
            format!("configuration profile valid={valid}"),
        )
        .await;

    (StatusCode::OK, Json(VerifyConfigProfileResponse { valid })).into_response()
}

pub async fn serve(addr: SocketAddr, api_token: String) {
    let state = AppState::bootstrap_with_bind(api_token, addr);
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

    #[tokio::test]
    async fn protocol_toggle_requires_auth() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PATCH")
                    .uri("/v1/protocols/airplay")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"enabled":false}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn update_policy_rejects_invalid_rotation() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PATCH")
                    .uri("/v1/policy")
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"rotation_degrees":45}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_policy_rejects_empty_target_display() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PATCH")
                    .uri("/v1/policy")
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"target_display":"  "}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn accept_stopped_session_is_rejected() {
        let app = app(AppState::bootstrap("token".into()));
        let seeded_id = seeded_session_id(&app).await;

        let _stopped = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{seeded_id}/stop"))
                    .header("authorization", "Bearer token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{seeded_id}/accept"))
                    .header("authorization", "Bearer token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn generate_pairing_pin_requires_auth() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/v1/pairing/pin")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn signed_profile_round_trip_verifies() {
        let app = app(AppState::bootstrap("token".into()));

        let sign_payload = serde_json::json!({
            "name": "test-profile",
            "policy": {
                "acceptance": "ask",
                "max_sessions": 2,
                "queue_policy": "first-in",
                "audio_output_device": "default-speaker",
                "display": {
                    "target_display": "display-1",
                    "scaling_mode": "fit",
                    "rotation_degrees": 0,
                    "preserve_aspect_ratio": true
                }
            },
            "operator": {
                "device_name": "Lab Receiver",
                "pin_policy": "always",
                "network_visibility": "lan"
            }
        });

        let sign_response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/v1/config-profiles/sign")
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(sign_payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(sign_response.status(), StatusCode::OK);
        let signed_bytes = sign_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();

        let verify_response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/v1/config-profiles/verify")
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        String::from_utf8(signed_bytes.to_vec()).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(verify_response.status(), StatusCode::OK);
        let body = verify_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        assert!(String::from_utf8(body.to_vec())
            .unwrap()
            .contains("\"valid\":true"));
    }

    #[tokio::test]
    async fn operator_settings_reject_empty_device_name() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PATCH")
                    .uri("/v1/operator/settings")
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"device_name":"  "}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn reconnect_session_endpoint_handles_jitter_drop() {
        let app = app(AppState::bootstrap("token".into()));
        let seeded_id = seeded_session_id(&app).await;

        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{seeded_id}/reconnect"))
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"jitter_ms":200,"dropped":true}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(String::from_utf8(body.to_vec())
            .unwrap()
            .contains("\"resumed\":true"));
    }

    #[tokio::test]
    async fn performance_and_diagnostics_endpoints_are_available() {
        let app = app(AppState::bootstrap("token".into()));

        let perf = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/performance/report")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(perf.status(), StatusCode::OK);

        let diag = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/diagnostics/bundle")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(diag.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn update_policy_rejects_invalid_performance_ranges() {
        let app = app(AppState::bootstrap("token".into()));

        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("PATCH")
                    .uri("/v1/policy")
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"target_latency_ms":5}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PATCH")
                    .uri("/v1/policy")
                    .header("authorization", "Bearer token")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"max_bitrate_mbps":200}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn preview_state_endpoint_tracks_stream_lifecycle() {
        let app = app(AppState::bootstrap("token".into()));
        let seeded_id = seeded_session_id(&app).await;

        let connecting = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/preview/state")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(connecting.status(), StatusCode::OK);
        let body = connecting.into_body().collect().await.unwrap().to_bytes();
        let preview: PreviewStateResponse = serde_json::from_slice(&body).unwrap();
        assert!(matches!(
            preview.stream_state,
            PreviewStreamState::Connecting
        ));

        let _ = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{seeded_id}/accept"))
                    .header("authorization", "Bearer token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let live = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/preview/state")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = live.into_body().collect().await.unwrap().to_bytes();
        let preview: PreviewStateResponse = serde_json::from_slice(&body).unwrap();
        assert!(matches!(preview.stream_state, PreviewStreamState::Live));

        let _ = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{seeded_id}/stop"))
                    .header("authorization", "Bearer token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let no_stream = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/preview/state")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = no_stream.into_body().collect().await.unwrap().to_bytes();
        let preview: PreviewStateResponse = serde_json::from_slice(&body).unwrap();
        assert!(matches!(
            preview.stream_state,
            PreviewStreamState::NoActiveStream
        ));
    }

    #[tokio::test]
    async fn accept_missing_session_does_not_stop_existing_active_session() {
        let app = app(AppState::bootstrap("token".into()));

        let seeded_id = seeded_session_id(&app).await;
        let _accepted = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{seeded_id}/accept"))
                    .header("authorization", "Bearer token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{}/accept", Uuid::new_v4()))
                    .header("authorization", "Bearer token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let response = app
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
        let seeded = sessions.into_iter().find(|s| s.id == seeded_id).unwrap();
        assert_eq!(seeded.status, SessionStatus::Active);
    }
    #[tokio::test]
    async fn connect_instructions_include_operator_name_and_protocol_hints() {
        let app = app(AppState::bootstrap("token".into()));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/connect/instructions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let payload: ConnectInstructionsResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(payload.receiver_name, "Air Sender Receiver");
        assert_eq!(
            payload.local_url,
            "http://127.0.0.1:9760/v1/connect/instructions"
        );
        assert_eq!(payload.protocol_hints.len(), 3);
    }

    #[tokio::test]
    async fn connect_instructions_use_configured_bind_address() {
        let bind = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 8)), 9988);
        let app = app(AppState::bootstrap_with_bind("token".into(), bind));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/connect/instructions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let payload: ConnectInstructionsResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            payload.local_url,
            "http://10.0.0.8:9988/v1/connect/instructions"
        );
    }

    #[tokio::test]
    async fn pairing_pin_state_returns_generated_pin() {
        let app = app(AppState::bootstrap("token".into()));
        let _ = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/v1/pairing/pin")
                    .header("authorization", "Bearer token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/v1/pairing/pin")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let payload: PairingPinState = serde_json::from_slice(&body).unwrap();
        assert!(payload.enabled);
        assert!(payload.pin.is_some());
    }
}
