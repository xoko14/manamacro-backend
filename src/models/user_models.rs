use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct UserRequest{
    pub username: String,
    pub display_name: String,
    pub password: String,
}