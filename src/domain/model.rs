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

impl ProblemSummary {
    pub fn new(
        title: String,
        writer: String,
        tags: Vec<String>,
        languages: Vec<Language>,
    ) -> ProblemSummary {
        let now = time::now().to_timespec().sec;

        ProblemSummary {
            id: ulid::Ulid::new().to_string(),
            title,
            created_at: now,
            updated_at: now,
            writer,
            tags,
            languages,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Problem {
    #[serde(flatten)]
    pub summary: ProblemSummary,
    pub content: String,
    pub attachments: Vec<ProblemAttachment>,
}

impl Problem {
    pub fn new(title: String, content: String, writer: String) -> Problem {
        Problem {
            summary: ProblemSummary::new(title, writer, vec![], vec![]),
            content,
            attachments: vec![],
        }
    }
}
