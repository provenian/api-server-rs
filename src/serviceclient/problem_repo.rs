use debil::*;

#[derive(Table)]
#[sql(table_name = "problem_record", sql_type = "debil_mysql::MySQLValue")]
pub struct ProblemRecord {
    #[sql(size = 50, primary_key = true)]
    id: String,
    #[sql(size = 256)]
    title: String,
    created_at: i64,
    updated_at: i64,
    #[sql(size = 50, not_null = true)]
    writer: String,
}

#[derive(Table)]
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

#[derive(Table)]
#[sql(
    table_name = "problem_language_relation",
    sql_type = "debil_mysql::MySQLValue"
)]
pub struct ProblemLanguageRelation {
    #[sql(size = 50, not_null = true)]
    problem_id: String,
    #[sql(size = 50, not_null = true)]
    language: String,
}
