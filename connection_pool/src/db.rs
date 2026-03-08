use std::env;
use dotenvy::dotenv;
use diesel::prelude::*;
 use diesel::r2d2::{Pool,ConnectionManager};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("erro ao encontrar database_url");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("erro ao criar a database pool")
}
