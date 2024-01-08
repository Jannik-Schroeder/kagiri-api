use diesel::RunQueryDsl;
use diesel::prelude::*;
use regex::Regex;
use db::establish_connection;
use crate::{db, schema};
use crate::schema::users::dsl::users;

pub fn check_email(email: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    re.is_match(email)
}

pub fn check_password(password: &str) -> bool {
    let re_letter = Regex::new(r".*[A-Za-z].*").unwrap();
    let re_number = Regex::new(r".*[0-9].*").unwrap();
    let re_length = Regex::new(r".{8,}").unwrap();
    re_letter.is_match(password) && re_number.is_match(password) && re_length.is_match(password)
}

pub fn check_email_exist(email: &str) -> bool {
    let mut connection = establish_connection();
    let email_exists: bool = diesel::select(diesel::dsl::exists(users.filter(schema::users::dsl::email.eq(email))))
        .get_result(&mut connection)
        .unwrap_or(false);
    email_exists
}