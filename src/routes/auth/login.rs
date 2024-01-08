use cookie::{Cookie, time};
use rocket::serde::json::{json, Value};
use crate::models::auth::NewUser;
use crate::utils::hash::verify_password;
use crate::utils::jwt::create_token;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::status::Custom;
use rocket::http::CookieJar;

#[post("/auth/login", format = "json", data = "<user>")]
pub async fn login(
    user: Json<NewUser>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    let new_user = NewUser {
        email: user.email.clone(),
        password: user.password.clone(),
    };

    if new_user.email.is_empty() || new_user.password.is_empty() {
        return Err(Custom(
            Status::BadRequest,
            Json(json!({"success": false, "message": "Email or password is empty"})),
        ));
    }

    if !verify_password(new_user.password.clone(), &new_user.email) {
        return Err(Custom(
            Status::BadRequest,
            Json(json!({"success": false, "message": "Password is incorrect"})),
        ));
    }

    // Create a JWT token for the user
    let token = create_token(&new_user.email);
    println!("Token: {}", &token); // print the token to console

    // Create a new cookie with the specified settings
    let cookie = Cookie::build(("jwt_kagiri", token.clone()))
        .secure(false) // If you are on a non-secure (http) connection, set this to false
        .same_site(cookie::SameSite::Lax) // or SameSite::None
        .http_only(true)
        .path("/")
        .max_age(time::Duration::hours(24));

    println!("Cookie: {:?}", &cookie); // print the cookie to console

    cookies.add(cookie);

    Ok(Json(json!({"success": true, "message": "Login successful"})))

}