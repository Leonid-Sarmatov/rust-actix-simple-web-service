use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Error};

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password: String,
    pub user_info: String,
}

pub async fn select_all_data(pool: &Pool<Postgres>) -> Result<Vec<User>, Error> {
    let users: Vec<User> = sqlx::query_as("SELECT * FROM users_table;")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn create_table(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query("CREATE TABLE IF NOT EXIST users_table (id integer PRIMARY KEY GENERATED BY DEFAULT AS IDENTITY, user_name VARCHAR(255), password VARCHAR(255), user_info VARCHAR(255));")
        .fetch_all(pool)
        .await?;
    
    Ok(())
}