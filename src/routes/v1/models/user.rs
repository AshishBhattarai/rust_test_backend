use serde::Serialize;
use uuid::Uuid;

use crate::db::models::User;

// Common user Dtos

#[derive(Serialize)]
pub struct UserDto {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

impl UserDto {
    pub fn new(user: User) -> Self {
        UserDto {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.username,
        }
    }
}
