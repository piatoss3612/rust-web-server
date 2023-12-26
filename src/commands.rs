use diesel_async::{AsyncConnection, AsyncPgConnection};
use argon2::{password_hash::SaltString, password_hash::rand_core::OsRng, Argon2, PasswordHasher};

use crate::{
    models::NewUser,
    repositories::{RoleRepository, UserRepository},
};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Error connecting to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut conn = load_db_connection().await;

    // salt and hash password
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    let new_user = NewUser { username, password: password_hash.to_string() };
    let user = UserRepository::create(&mut conn, new_user, role_codes)
        .await
        .unwrap();

    println!("Created user: {:?}", user);

    let roles = RoleRepository::find_by_user(&mut conn, &user)
        .await
        .unwrap();

    println!("User roles: {:?}", roles);
}

pub async fn list_users() {
    let mut conn = load_db_connection().await;

    let users = UserRepository::find_with_roles(&mut conn).await.unwrap();

    for user in users {
        println!("{:?}", user);
    }
}

pub async fn delete_user(id: i32) {
    let mut conn = load_db_connection().await;

    UserRepository::delete(&mut conn, id).await.unwrap();
}
