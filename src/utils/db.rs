use diesel::prelude::*;
use crate::db::establish_connection;
use crate::schema::users::{email, password};
use crate::schema::users::dsl::users;

pub fn get_hashed_password(user_email: &String) -> String {
    let mut connection = establish_connection();
    let hashed_password: String = users
        .filter(email.eq(&user_email))
        .select(password)
        .first(&mut connection)
        .unwrap_or("".to_string());
    return hashed_password;
}