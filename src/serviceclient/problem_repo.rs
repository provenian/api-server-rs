use crate::domain::interface::IProblemRepository;
use crate::domain::model;
use crate::error::ServiceError;
use async_trait::async_trait;
use debil::*;

#[derive(Table)]
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
        tags: Vec<ProblemTagRelation>,
        attachments: Vec<ProblemLanguageRelation>,
    ) -> model::Problem {
        model::Problem {
            summary: model::ProblemSummary {
                id: self.id,
                title: self.title,
                created_at: self.created_at,
                updated_at: self.updated_at,
                writer: self.writer,
                tags: tags.into_iter().map(|t| t.tag).collect::<Vec<_>>(),
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
    conn_pool: mysql_async::Pool,
}

impl ProblemRepository {
    pub fn new(conn_pool: mysql_async::Pool) -> ProblemRepository {
        ProblemRepository {
            conn_pool: conn_pool,
        }
    }
}

#[async_trait]
impl IProblemRepository for ProblemRepository {
    async fn save(&self, problem: model::Problem) -> Result<(), ServiceError> {
        let (record, tags, languages) = ProblemRecord::from_model(problem);

        let mut conn = debil_mysql::DebilConn::from_conn(
            self.conn_pool
                .get_conn()
                .await
                .map_err(ServiceError::DBError)?,
        );

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
        Ok(vec![])
    }

    async fn list_by_tag(&self, tag: String) -> Result<Vec<model::ProblemSummary>, ServiceError> {
        Ok(vec![])
    }

    async fn find_by_id(&self, key: String) -> Result<Option<model::Problem>, ServiceError> {
        let mut conn = debil_mysql::DebilConn::from_conn(
            self.conn_pool
                .get_conn()
                .await
                .map_err(ServiceError::DBError)?,
        );

        let record = conn
            .first::<ProblemRecord>()
            .await
            .map_err(ServiceError::DBError)?;

        Ok(record.map(|r| r.to_model(vec![], vec![])))
    }
}
