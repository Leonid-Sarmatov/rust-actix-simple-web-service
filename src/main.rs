mod services;
use services::{hello_handler, User};

use actix_web::{web::Data, App, HttpServer};
use dotenv::{dotenv};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

const DB_URL: &str = "postgresql://postgres:USPEH@localhost:5432/users";

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //dotenv().ok();
    //let db_url = std::env::var("DATABASE_URL").expect("Database URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL)
        .await
        .expect("Can not connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone()}))
            .service(hello_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
