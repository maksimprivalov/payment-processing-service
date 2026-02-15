use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn init_db(database_url: &str) -> Db {
    Pool::<Postgres>::connect(database_url)
        .await
        .expect("Failed to connect to database")
}