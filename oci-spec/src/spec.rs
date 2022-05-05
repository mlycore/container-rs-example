use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Refers to: https://github.com/opencontainers/runtime-spec/blob/main/runtime.md 
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Spec {
//     pub oci_version: String,
//     pub id: String,
//     pub status: ContainerStatus,
//     pub bundle: String,
//     pub pid: u32,
//     pub annotations: Option<HashMap<String, String>>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub enum ContainerStatus {
//     Creating,
//     Created,
//     Running,
//     Stopped,
// }

// Refers to: https://github.com/opencontainers/runtime-spec/blob/main/config.md
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub oci_version: String,
    pub root: Root,
    pub mounts: Option<Vec<Mount>>,
    pub process: Option<Process>,
    pub hostname: Option<String>,
    pub hooks: Option<Hooks>,
    pub linux: Option<Linux>,
    pub annotations: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub path: String,
    pub readonly: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    pub destination: String,
    pub source: Option<String>,
    pub options: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub mount_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    pub terminal: Option<bool>,
    pub console_size: Option<ConsoleSize>,
    pub cwd: String,
    pub env: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    pub command_line: Option<String>,
    pub rlimits: Option<Vec<Rlimit>>,
    pub apparmor_profile: Option<String>,
    pub capabilities: Option<Capabilities>,
    pub user: Option<User>,
    pub no_new_privileges: Option<bool>,
    pub oom_score_adj: Option<i64>,
    pub selinux_label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleSize {
    pub height: u8,
    pub width: u8,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: i64,
    pub gid: i64,
    pub additional_gids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlimit {
    #[serde(rename = "type")]
    pub type_field: String,
    pub hard: i64,
    pub soft: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    pub bounding: Option<Vec<String>>,
    pub permitted: Option<Vec<String>>,
    pub inheritable: Option<Vec<String>>,
    pub effective: Option<Vec<String>>,
    pub ambient: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hooks {
    pub prestart: Option<Vec<Hook>>,
    pub create_runtime: Option<Vec<Hook>>,
    pub create_container: Option<Vec<Hook>>,
    pub start_container: Option<Vec<Hook>>,
    pub poststart: Option<Vec<Hook>>,
    pub poststop: Option<Vec<Hook>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hook {
    pub path: String,
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub timeout: Option<i64>,
}

// Refers to https://github.com/opencontainers/runtime-spec/blob/main/config-linux.md
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Linux {
    pub namespaces: Option<Vec<Namespace>>,
    pub uid_mappings: Option<Vec<UidMapping>>,
    pub devices: Option<Vec<Device>>,
    pub cgroups_path: Option<String>,
    pub gid_mappings: Option<Vec<UidMapping>>,
    pub sysctl: Option<HashMap<String, String>>,
    pub resources: Option<Resources>,
    pub rootfs_propagation: Option<String>,
    pub seccomp: Option<Seccomp>,
    pub masked_paths: Option<Vec<String>>,
    pub readonly_paths: Option<Vec<String>>,
    pub mount_label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub network: Option<Network>,
    pub pids: Option<Pids>,
    pub hugepage_limits: Option<Vec<HugepageLimit>>,
    pub memory: Option<Memory>,
    pub cpu: Option<Cpu>,
    pub devices: Option<Vec<DeviceResource>>,
    #[serde(rename = "blockIO")]
    pub block_io: Option<BlockIo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceResource {
    pub allow: Option<bool>,
    pub access: Option<String>,
    #[serde(rename = "type")]
    pub device_type: Option<String>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    #[serde(rename = "type")]
    pub namespace: String,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UidMapping {
    #[serde(rename = "containerID")]
    pub container_id: i64,
    #[serde(rename = "hostID")]
    pub host_id: i64,
    pub size: i64,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub path: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub major: u64,
    pub minor: u64,
    pub file_mode: Option<i64>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    pub limit: Option<i64>,
    pub reservation: Option<i64>,
    pub swap: Option<i64>,
    pub kernel: Option<i64>,
    #[serde(rename = "kernelTCP")]
    pub kernel_tcp: Option<i64>,
    pub swappiness: Option<i64>,
    #[serde(rename = "disableOOMKiller")]
    pub disable_oomkiller: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cpu {
    pub shares: Option<i64>,
    pub quota: Option<i64>,
    pub period: Option<i64>,
    pub realtime_runtime: Option<i64>,
    pub realtime_period: Option<i64>,
    pub cpus: Option<String>,
    pub mems: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockIo {
    pub weight: Option<i64>,
    pub leaf_weight: Option<i64>,
    pub weight_device: Option<Vec<WeightDevice>>,
    pub throttle_read_bps_device: Option<Vec<ThrottleReadBpsDevice>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iopsdevice: Option<Vec<ThrottleWriteIopsdevice>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightDevice {
    pub major: i64,
    pub minor: i64,
    pub weight: Option<i64>,
    pub leaf_weight: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThrottleReadBpsDevice {
    pub major: i64,
    pub minor: i64,
    pub rate: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThrottleWriteIopsdevice {
    pub major: i64,
    pub minor: i64,
    pub rate: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HugepageLimit {
    pub page_size: String,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    #[serde(rename = "classID")]
    pub class_id: Option<i64>,
    pub priorities: Option<Vec<Priority>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    pub name: String,
    pub priority: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pids {
    pub limit: Option<i64>,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Seccomp {
    pub default_action: String,
    pub architectures: Option<Vec<String>>,
    pub syscalls: Option<Vec<Syscall>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Syscall {
    pub names: Vec<String>,
    pub action: String,
}