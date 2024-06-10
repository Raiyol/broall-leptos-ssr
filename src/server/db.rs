use std::env;

use diesel::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

// connect to MySql DB
pub fn init_pool() -> DbPool {
    let conn_url = env::var("DATABASE_URL").expect("Failed to get value of DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(&conn_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
