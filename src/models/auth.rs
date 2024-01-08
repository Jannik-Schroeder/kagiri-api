use diesel::Insertable;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}