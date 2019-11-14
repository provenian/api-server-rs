use crate::domain::interface::IProblemRepository;
use crate::domain::model;
use crate::error::ServiceError;
use crate::infra::conn_pool::ConnPool;
use async_trait::async_trait;
use debil::*;

#[derive(Table, Clone)]
#[sql(table_name = "problem_record", sql_type = "debil_mysql::MySQLValue")]
pub struct ProblemRecord {
    #[sql(size = 50, primary_key = true)]
    id: String,
    #[sql(size = 256)]
    title: String,
    content: String,
    created_at: i64,
    updated_at: i64,
    #[sql(size = 50, not_null = true)]
    writer: String,
}

impl ProblemRecord {
    fn from_model(
        data: model::Problem,
    ) -> (
        ProblemRecord,
        Vec<ProblemTagRelation>,
        Vec<ProblemLanguageRelation>,
    ) {
        let problem_id = data.summary.id.clone();
        let problem_id_clone = problem_id.clone();
        let record = ProblemRecord {
            id: data.summary.id,
            title: data.summary.title,
            content: data.content,
            created_at: data.summary.created_at,
            updated_at: data.summary.updated_at,
            writer: data.summary.writer,
        };
        let tags = data
            .summary
            .tags
            .into_iter()
            .map(move |t| ProblemTagRelation {
                problem_id: problem_id.clone(),
                tag: t,
            })
            .collect::<Vec<_>>();
        let languages = data
            .attachments
            .iter()
            .map(move |attachment| ProblemLanguageRelation {
                problem_id: problem_id_clone.clone(),
                language: serde_json::to_string(&attachment.language).unwrap(),
                version: attachment.version.clone(),
                content: attachment.content.clone(),
            })
            .collect::<Vec<_>>();

        (record, tags, languages)
    }

    fn to_model(
        self,
        tags: Vec<String>,
        attachments: Vec<ProblemLanguageRelation>,
    ) -> model::Problem {
        model::Problem {
            summary: model::ProblemSummary {
                id: self.id,
                title: self.title,
                created_at: self.created_at,
                updated_at: self.updated_at,
                writer: self.writer,
                tags: tags,
                languages: attachments
                    .iter()
                    .map(|r| serde_json::from_str(&r.language).unwrap_or(model::Language::Unknown))
                    .collect::<Vec<_>>(),
            },
            content: self.content,
            attachments: attachments
                .into_iter()
                .map(|r| model::ProblemAttachment {
                    language: serde_json::from_str(&r.language).unwrap_or(model::Language::Unknown),
                    version: r.version,
                    content: r.content,
                })
                .collect::<Vec<_>>(),
        }
    }

    fn to_model_summary(
        self,
        tags: Vec<String>,
        languages: Vec<model::Language>,
    ) -> model::ProblemSummary {
        model::ProblemSummary {
            id: self.id,
            title: self.title,
            created_at: self.created_at,
            updated_at: self.updated_at,
            writer: self.writer,
            tags: tags,
            languages: languages,
        }
    }
}

#[derive(Table, Clone)]
#[sql(
    table_name = "problem_tag_relation",
    sql_type = "debil_mysql::MySQLValue"
)]
pub struct ProblemTagRelation {
    #[sql(size = 50, not_null = true)]
    problem_id: String,
    #[sql(size = 50, not_null = true)]
    tag: String,
}

#[derive(Table, Clone)]
#[sql(
    table_name = "problem_language_relation",
    sql_type = "debil_mysql::MySQLValue"
)]
pub struct ProblemLanguageRelation {
    #[sql(size = 50, not_null = true)]
    problem_id: String,
    #[sql(size = 50, not_null = true)]
    language: String,
    #[sql(size = 50)]
    version: String,
    content: String,
}

pub struct ProblemRepository {
    db: ConnPool,
}

impl ProblemRepository {
    pub fn new(db: ConnPool) -> ProblemRepository {
        ProblemRepository { db }
    }
}

pub struct JoinedProblemView {
    problem: ProblemRecord,
    tag: String,
}

impl debil::SQLMapper for JoinedProblemView {
    type ValueType = debil_mysql::MySQLValue;

    fn map_from_sql(values: std::collections::HashMap<String, Self::ValueType>) -> Self {
        let tag = String::deserialize(values["tag"].clone());
        let problem = debil::map_from_sql::<ProblemRecord>(values);

        JoinedProblemView { problem, tag }
    }
}

#[async_trait]
impl IProblemRepository for ProblemRepository {
    async fn save(&self, problem: model::Problem) -> Result<(), ServiceError> {
        let (record, tags, languages) = ProblemRecord::from_model(problem);

        let mut conn = self.db.get_conn().await?;
        conn.save::<ProblemRecord>(record)
            .await
            .map_err(ServiceError::DBError)?;
        conn.save_all::<ProblemTagRelation>(tags)
            .await
            .map_err(ServiceError::DBError)?;
        conn.save_all::<ProblemLanguageRelation>(languages)
            .await
            .map_err(ServiceError::DBError)?;

        Ok(())
    }

    async fn list(&self) -> Result<Vec<model::ProblemSummary>, ServiceError> {
        let mut conn = self.db.get_conn().await?;

        let problems = conn
            .load_with2::<ProblemRecord, JoinedProblemView>(
                debil::QueryBuilder::new()
                    .left_join(
                        SQLTable::table_name(std::marker::PhantomData::<ProblemTagRelation>),
                        ("id", "problem_id"),
                    )
                    .group_by(vec!["problem_record.id"])
                    .append_selects(vec!["GROUP_CONCAT(tag) as tag"]),
            )
            .await?;

        Ok(problems
            .into_iter()
            .map(|v| v.problem.to_model_summary(vec![v.tag], vec![]))
            .collect::<Vec<_>>())
    }

    async fn list_by_tag(&self, tag: String) -> Result<Vec<model::ProblemSummary>, ServiceError> {
        Ok(vec![])
    }

    async fn find_by_id(&self, key: String) -> Result<model::Problem, ServiceError> {
        let mut conn = self.db.get_conn().await?;

        let cond = vec![format!("id = {}", key)];
        let record = conn
            .first_with::<ProblemRecord>(debil::QueryBuilder::new().wheres(cond))
            .await?;

        Ok(record.to_model(vec![], vec![]))
    }
}
