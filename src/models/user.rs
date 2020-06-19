use crate::db;
use crate::error_handler::CustomError;
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use argon2::Config;
use rand::Rng;

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String   
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[table_name = "users"]
pub struct UserDTO {
    pub firstname: String,
    pub lastname: String,
    pub email: String
}


#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let users = users::table.load::<User>(&conn)?;
        Ok(users)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = users::table.filter(users::id.eq(id)).first(&conn)?;
        Ok(user)
    }

    pub fn update(id: i32, user: User) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&conn)?;
            Ok(user)
    }

    pub fn create(user: User) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let mut user = User::from(user);
        user.hash_password()?;
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)           
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&conn)?;
        Ok(res)
    }

    pub fn hash_password(&mut self) -> Result<(), CustomError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| CustomError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())
    }

    pub fn verify_password(&self, password: &[u8]) -> Result<bool, CustomError> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| CustomError::new(500, format!("Failed to verify password: {}", e)))
    }

    pub fn login(username: String, password: String) -> Result<bool, CustomError> {

    }

}