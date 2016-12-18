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

