use serde::*;

#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Role {
    Writer,
    User,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub role: Role,
}
