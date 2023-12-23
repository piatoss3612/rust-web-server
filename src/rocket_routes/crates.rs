use crate::models::{Crate, NewCrate};
use crate::repositories::CrateRepository;
use crate::rocket_routes::{server_error, DbConn};
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100)
        .await
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id)
        .await
        .map(|an_crate| json!(an_crate))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/crates/<id>", format = "json", data = "<an_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    an_crate: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, an_crate.into_inner())
        .await
        .map(|an_crate| json!(an_crate))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}
