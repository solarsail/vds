use serde::{Deserialize,Deserializer};
use serde_json::value;

#[derive(Serialize, Deserialize, Debug)]
pub enum DiskConfig {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "MANUAL")]
    Manual,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerState {
    #[serde(rename(deserialize="active"))]
    Active,
    #[serde(rename(deserialize="stopped"))]
    Stopped,
    Building,
    Deleted,
    Error,
    HardReboot,
    Migrating,
    Password,
    Paused,
    Reboot,
    Rebuild,
    Rescued,
    Resized,
    RevertResize,
    SoftDeleted,
    Suspended,
    Unknown,
    VeriryResize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PowerState {
    NoState = 0,
    Running = 1,
    Paused = 3,
    Shutdown = 4,
    Crashed = 6,
    Suspended = 7,
}

fn parse_power_state<D>(d: &mut D) -> Result<PowerState, D::Error> where D: Deserializer {
    if let Ok(t) = u64::deserialize(d) {
        let ps = match t {
            1 => PowerState::Running,
            3 => PowerState::Paused,
            4 => PowerState::Shutdown,
            6 => PowerState::Crashed,
            7 => PowerState::Suspended,
            _ => PowerState::NoState,
        };
        Ok(ps)
    } else {
        Ok(PowerState::NoState)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerUsage {
    pub ended_at: Option<String>,
    pub flavor: String,
    pub hours: f64,
    pub instance_id: String,
    pub local_gb: u32,
    pub memory_mb: u32,
    pub name: String,
    pub started_at: String,
    pub state: ServerState,
    pub tenant_id: String,
    pub uptime: u32,
    pub vcpus: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TenantUsage {
    pub server_usages: Vec<ServerUsage>,
    pub start: String,
    pub stop: String,
    pub tenant_id: String,
    pub total_hours: f64,
    pub total_memory_mb_usage: f64,
    pub total_local_gb_usage: f64,
    pub total_vcpus_usage: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub href: String,
    pub rel: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub addr: String,
    #[serde(rename(deserialize="OS-EXT-IPS-MAC:mac_addr"))]
    pub mac_addr: String,
    #[serde(rename(deserialize="OS-EXT-IPS:type"))]
    pub addr_type: String,
    pub version: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Addresses {
    pub private: Vec<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flavor {
    pub id: String,
    #[serde(skip_serializing)]
    pub links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: String,
    #[serde(skip_serializing)]
    pub links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Volume {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityGroup {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    #[serde(rename(deserialize="accessIPv4"))]
    pub access_ipv4: String,
    #[serde(rename(deserialize="accessIPv6"))]
    pub access_ipv6: String,
    pub addresses: Addresses,
    pub created: String,
    pub flavor: Flavor,
    #[serde(rename(deserialize="hostId"))]
    pub host_id: String,
    pub id: String,
    pub image: Image,
    pub key_name: Option<String>,
    #[serde(skip_serializing)]
    pub links: Vec<Link>,
    pub metadata: value::Map<String, value::Value>,//
    pub name: String,
    pub config_drive: String,
    #[serde(rename(deserialize="OS-DCF:diskConfig"))]
    pub disk_config: DiskConfig,
    #[serde(rename(deserialize="OS-EXT-AZ:availability_zone"))]
    pub availability_zone: String,
    #[serde(rename(deserialize="OS-SRV-USG:launched_at"))]
    pub launched_at: Option<String>,
    #[serde(rename(deserialize="OS-SRV-USG:terminated_at"))]
    pub terminated_at: Option<String>,
    #[serde(rename(deserialize="os-extended-volumes:volumes_attached"))]
    pub volumes_attached: Vec<Volume>,
    #[serde(rename(deserialize="OS-EXT-STS:power_state"), deserialize_with="parse_power_state")]
    pub power_state: PowerState,
    #[serde(rename(deserialize="OS-EXT-STS:task_state"))]
    pub task_state: Option<String>,
    #[serde(rename(deserialize="OS-EXT-STS:vm_state"))]
    pub vm_state: ServerState,
    #[serde(default)]
    pub progress: u8,
    pub security_groups: Vec<SecurityGroup>,
    pub status: String,
    pub tenant_id: String,
    pub updated: String,
    pub user_id: String,
}

