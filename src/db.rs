use std::env;
use dotenvy::dotenv;
use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não encontrado!");
    SqliteConnection::establish(&database_url).expect("erro ao conectar com o banco de dados!")
}