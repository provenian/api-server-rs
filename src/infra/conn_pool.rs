#[derive(Clone)]
pub struct ConnPool(mysql_async::Pool);

impl ConnPool {
    pub fn from_url(database_url: &str) -> ConnPool {
        ConnPool(mysql_async::Pool::from_url(database_url).unwrap())
    }

    pub async fn get_conn(&self) -> Result<debil_mysql::DebilConn, debil_mysql::Error> {
        Ok(debil_mysql::DebilConn::from_conn(self.0.get_conn().await?))
    }
}
