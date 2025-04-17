pub mod create_user;
pub mod login;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct User {
    id: i32,
    email: String,
    name: String,
    last_name: String,
    driver: bool,
    hashed_password: String,
}