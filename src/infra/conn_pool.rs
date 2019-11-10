use actix::prelude::*;

#[derive(Clone)]
pub struct DBConnector(Addr<ConnPool>);

#[derive(Fail, Debug)]
pub enum DBConnectorError {
    #[fail(display = "DB Error: {}", _0)]
    DBError(#[fail(cause)] debil_mysql::Error),

    #[fail(display = "Actor Error: {}", _0)]
    MailboxError(#[fail(cause)] actix::MailboxError),
}

impl DBConnector {
    pub fn new(database_url: String) -> DBConnector {
        // mysql_async側でConnectionPoolの仕組みは持っているのでthreadは少なくてよいはず
        DBConnector(SyncArbiter::start(1, move || {
            ConnPool::from_url(database_url.clone())
        }))
    }

    pub async fn get_conn(&self) -> Result<debil_mysql::DebilConn, DBConnectorError> {
        use futures::compat::Future01CompatExt;

        let conn = self
            .0
            .send(GetConn)
            .compat()
            .await
            .map_err(DBConnectorError::MailboxError)?
            .map_err(DBConnectorError::DBError)?;

        Ok(conn)
    }
}

#[derive(Clone)]
pub struct ConnPool(mysql_async::Pool);

impl ConnPool {
    pub fn from_url(database_url: String) -> ConnPool {
        ConnPool(mysql_async::Pool::from_url(database_url).unwrap())
    }
}

impl Actor for ConnPool {
    type Context = SyncContext<Self>;
}

pub struct GetConn;

impl Message for GetConn {
    type Result = Result<debil_mysql::DebilConn, debil_mysql::Error>;
}

impl Handler<GetConn> for ConnPool {
    type Result = Result<debil_mysql::DebilConn, debil_mysql::Error>;

    fn handle(&mut self, _: GetConn, _: &mut Self::Context) -> Self::Result {
        actix_rt::Runtime::new()
            .unwrap()
            .block_on(futures::compat::Compat::new(self.0.get_conn()))
            .map(debil_mysql::DebilConn::from_conn)
            .map_err(From::from)
    }
}
