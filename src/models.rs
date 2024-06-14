use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

impl Todo {
    pub fn new(description: String) -> Todo {
        // Step 1: Initialize a new Todo object with the given description
    }
}

pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub async fn new(database_url: &str) -> Self {
        // Step 1: Connect to the database using the provided URL
        // Step 2: Return an instance of AppState with the connected database pool
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

impl Category {
    pub fn new(name: String) -> Category {
        // Step 1: Initialize a new Category object with the given name
    }
}

impl Category {
    pub async fn fetch_all(
        pool: &SqlitePool
    ) -> Result<Vec<Category>, sqlx::Error> {
        // Step 1: Write the SQL query to fetch all categories
        // Step 2: Execute the query and collect results
        // Step 3: Return the list of categories
    }

    pub async fn count_todos(
        id: i32,
        pool: &SqlitePool
    ) -> Result<i64, sqlx::Error> {
        // Step 1: Write the SQL query to count todos in the given category
        // Step 2: Execute the query and fetch the count
        // Step 3: Return the count
    }
}
