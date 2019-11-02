use serde::*;
use std::collections::HashMap;

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

#[derive(Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Isabelle,
    Unknown,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProblemAttachment {
    pub language: Language,
    pub version: String,
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProblemSummary {
    pub id: String,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub writer: String,
    pub tags: Vec<String>,
    pub languages: Vec<Language>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Problem {
    #[serde(flatten)]
    pub summary: ProblemSummary,
    pub content: String,
    pub attachments: Vec<ProblemAttachment>,
}
