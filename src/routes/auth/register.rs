use rocket::serde::json::{Json, json, Value};
use crate::models::auth::NewUser;
use diesel::prelude::*;
use crate::schema::users::dsl::users;
use db::establish_connection;
use crate::db;
use crate::utils::hash::hash_password;
use crate::utils::validator::check_email;
use crate::utils::validator::check_password;
use crate::utils::validator::check_email_exist;

use rocket::http::Status;
use rocket::response::status::Custom;

#[post("/auth/register", format = "json", data = "<user>")]
pub async fn register(user: Json<NewUser>) -> Result<Json<Value>, Custom<Json<Value>>> {
    let new_user = NewUser {
        email: user.email.clone(),
        password: user.password.clone(),
    };

    if new_user.email.is_empty() || new_user.password.is_empty() {
        return Err(Custom(
            Status::BadRequest,
            Json(json!({"success": "false", "message": "Email or password is empty"})),
        ));
    }

    if check_email_exist(&new_user.email) {
        return Err(Custom(
            Status::BadRequest,
            Json(json!({"success": "false", "message": "Email already exists"})),
        ));
    }

    if !check_email(&new_user.email) {
        return Err(Custom(
            Status::BadRequest,
            Json(json!({"success": "false", "message": "Email is not valid"})),
        ));
    }

    if !check_password(&new_user.password) {
        return Err(Custom(
            Status::BadRequest,
            Json(json!({"success": "false", "message": "Password is too weak"})),
        ));
    }

    let new_user = NewUser {
        email: new_user.email.clone(),
        password: hash_password(new_user.password.clone(), &new_user.email),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut establish_connection())
        .map_err(|_| Custom(
            Status::InternalServerError,
            Json(json!({"success": "false", "message": "Internal server error"})),
        ))?;

    Ok(Json(json!({"success": "true"})))
}