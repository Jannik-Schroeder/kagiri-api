// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}
