use crate::{
    DeviceDescriptor, ReceiverPolicy, RecordingProfile, SessionDescriptor, StreamCapabilities,
};

pub trait ProtocolAdapter: Send + Sync {
    fn protocol_id(&self) -> &'static str;
    fn discover(&self) -> Vec<DeviceDescriptor>;
    fn capabilities(&self) -> StreamCapabilities;
}

pub trait SessionController: Send + Sync {
    fn list_sessions(&self) -> Vec<SessionDescriptor>;
}

pub trait CompositorEngine: Send + Sync {
    fn layout_mode(&self) -> &'static str;
}

pub trait Recorder: Send + Sync {
    fn start(&self, session_id: &str, profile: &RecordingProfile) -> bool;
    fn stop(&self, session_id: &str) -> bool;
}

pub trait PairingStore: Send + Sync {
    fn trust_device(&self, device_id: &str);
    fn is_trusted(&self, device_id: &str) -> bool;
}

pub trait PolicyEngine: Send + Sync {
    fn get_policy(&self) -> ReceiverPolicy;
}
