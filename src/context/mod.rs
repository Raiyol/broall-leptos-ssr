use diesel::r2d2::{self, ConnectionManager};
use diesel::MysqlConnection;
use std::sync::Arc;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct MyAppContext {
    pub pool: Arc<DbPool>,
}
