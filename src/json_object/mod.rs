#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub domain: Domain,
    pub id: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    pub id: String,
    #[serde(skip_serializing_if="String::is_empty")]
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
