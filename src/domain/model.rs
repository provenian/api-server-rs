use serde::*;

#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Writer,
    User,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthUser {
    #[serde(rename(deserialize = "https://github.com/myuon/provenian/roles"))]
    pub role: Vec<Role>,
}
