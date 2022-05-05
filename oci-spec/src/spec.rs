use std::collections::HashMap;

// Refers to: https://github.com/opencontainers/runtime-spec/blob/main/runtime.md 
pub struct Spec {
    pub oci_version: String,
    pub id: String,
    pub status: ContainerStatus,
    pub bundle: String,
    pub pid: u32,
    pub annotations: Option<HashMap<String, String>>,
}

pub enum ContainerStatus {
    Creating,
    Created,
    Running,
    Stopped,
}