use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{models::NewUser, repositories::UserRepository};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Error connecting to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut conn = load_db_connection().await;

    let new_user = NewUser { username, password };
    let user = UserRepository::create(&mut conn, new_user).await.unwrap();

    println!("Created user: {:?}", user);
}

pub async fn list_users() {
    let mut conn = load_db_connection().await;
}

pub async fn delete_user(id: i32) {
    let mut conn = load_db_connection().await;
}
