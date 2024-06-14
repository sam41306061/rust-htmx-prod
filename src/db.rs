use sqlx::SqlitePool;
use crate::models::Todo;


impl Todo {
    pub async fn save_new_todo(
        &self, 
        pool: &SqlitePool
    ) -> Result<(), sqlx::Error> {
        // Step 1: Write the SQL query to insert a new todo
        // Step 2: Execute the query with the Todo's data
        // Step 3: Return success
    }
}
