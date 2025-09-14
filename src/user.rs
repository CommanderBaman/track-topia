use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::db::Database;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> User {
        User { id, name }
    }
}

pub trait UserDatabase {
    fn create_user(&self, user_name: &str) -> Result<i32, rusqlite::Error>;
    fn get_user_by_id(&self, user_id: &i32) -> Result<User, rusqlite::Error>;
    fn get_user_by_name(&self, user_name: &str) -> Result<User, rusqlite::Error>;
}

impl UserDatabase for Database {
    fn create_user(&self, user_name: &str) -> Result<i32, rusqlite::Error> {
        let conn = self.get_connection();
        conn.execute_batch(format!("INSERT INTO users (name) VALUES ('{}');", user_name).as_str())
            .expect("user insert query should be successful");

        let user_id: i32 = conn
            .query_row(
                "SELECT id FROM users WHERE name = ?",
                params![user_name],
                |row| row.get(0),
            )
            .expect("user query should be successful");
        Ok(user_id)
    }

    fn get_user_by_id(&self, user_id: &i32) -> Result<User, rusqlite::Error> {
        let conn = self.get_connection();

        conn.query_row(
            "SELECT id, name FROM users WHERE id = ?",
            params![user_id],
            |row| {
                let id: i32 = row.get(0)?;
                let name: String = row.get(1)?;
                Ok(User::new(id, name))
            },
        )
    }

    fn get_user_by_name(&self, user_name: &str) -> Result<User, rusqlite::Error> {
        let conn = self.get_connection();

        conn.query_row(
            "SELECT id, name FROM users WHERE name = ?",
            params![user_name],
            |row| {
                let id: i32 = row.get(0)?;
                let name: String = row.get(1)?;
                Ok(User::new(id, name))
            },
        )
    }
}
