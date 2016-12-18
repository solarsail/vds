use serde::{Deserialize,Deserializer};


#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    pub id: String,
    pub name: String,
}

fn default_domain_id() -> String {
    "domain".to_string()
}

fn default_domain_name() -> String {
    "Domain".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(default)]
    pub domain: Domain,
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    #[serde(skip_serializing)]
    pub methods: Vec<String>,
    pub roles: Vec<Role>,
    pub expires_at: String,
    pub project: Project,
    pub user: User,
    pub issued_at: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Domain {
    #[serde(default="default_domain_id", skip_serializing_if="String::is_empty")]
    pub id: String,
    #[serde(default="default_domain_name", skip_serializing_if="String::is_empty")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    #[serde(skip_serializing_if="String::is_empty")]
    pub id: String,
    pub domain: Domain,
    #[serde(default, skip_serializing_if="String::is_empty")]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Password {
    pub user: User
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    pub methods: Vec<String>,
    pub password: Password,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub identity: Identity,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "stopped")]
    Stopped,
}

fn de_string<D>(d: &mut D) -> Result<String, D::Error> where D: Deserializer {
    if let Ok(s) = String::deserialize(d) {
        Ok(s)
    } else {
        Ok(String::new())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerUsage {
    #[serde(deserialize_with = "de_string")]
    pub ended_at: String,
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
pub struct TenantUsageLink {
    pub href: String,
    pub rel: String,
}

