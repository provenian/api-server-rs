use debil::*;

#[derive(Table)]
#[sql(table_name = "problem_record", sql_type = "debil_mysql::MySQLValue")]
pub struct ProblemRecord {
    #[sql(primary_key = true, size = 50)]
    id: String,
    #[sql(size = 256)]
    title: String,
    created_at: i64,
    updated_at: i64,
    #[sql(size = 50)]
    writer: String,
}

#[derive(Table)]
#[sql(
    table_name = "problem_tag_relation",
    sql_type = "debil_mysql::MySQLValue"
)]
pub struct ProblemTagRelation {
    problem_id: String,
    tag: String,
}

#[derive(Table)]
#[sql(
    table_name = "problem_language_relation",
    sql_type = "debil_mysql::MySQLValue"
)]
pub struct ProblemLanguageRelation {
    problem_id: String,
    language: String,
}
