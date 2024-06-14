// Import necessary libraries and modules
// use actix_web::{post, put, get, web, HttpResponse};
// use crate::models::{Todo, AppState};
// use serde::Deserialize;

// Define a structure for the create todo form
#[derive(Deserialize)]
pub struct CreateTodo {
    // description: String, // Field for todo description
}

// Define an async function to handle adding a new todo
#[post("/add_todo")]
async fn add_todo(
    // Define the data type for the AppState
    // data: web::Data<AppState>,
    
    // Define the form type to extract the CreateTodo structure
    // form: web::Form<CreateTodo>
) -> HttpResponse {
    // Step 1: Extract the todo description from the form
    // let description = form.into_inner().description;
    
    // Step 2: Create a new Todo object
    // let new_todo = Todo::new(description);
    
    // Step 3: Save the Todo object to the database
    // if let Err(err) = data.db.save_new_todo(new_todo).await {
        // Return an error response if saving fails
        // return HttpResponse::InternalServerError().body(format!("Failed to add todo: {}", err));
    // }

    // Step 4: Return a success response
    // HttpResponse::Ok().body("Todo added successfully")
}

// Define an async function to handle fetching todos
#[get("/get_todos")]
async fn get_todos(
    // Define the data type for the AppState
    // data: web::Data<AppState>
) -> HttpResponse {
    // Step 1: Fetch todos from the database
    // let todos = Todo::fetch_all(&data.db).await;
    
    // Step 2: Check for errors and return the appropriate response
    // match todos {
        // Ok(todos) => HttpResponse::Ok().json(todos),
        // Err(err) => HttpResponse::InternalServerError().body(format!("Failed to load todos: {}", err)),
    // }
}

// Define a structure for updating todo status
#[derive(Deserialize)]
struct UpdateTodoStatus {
    // id: i32, // Field for todo ID
}

// Define an async function to handle marking a todo as done
#[put("/mark_todo_done")]
async fn mark_todo_done(
    // Define the data type for the AppState
    // data: web::Data<AppState>,
    
    // Define the form type to extract the UpdateTodoStatus structure
    // form: web::Form<UpdateTodoStatus>
) -> HttpResponse {
    // Step 1: Extract the todo ID from the form
    // let todo_id = form.into_inner().id;
    
    // Step 2: Update the status of the todo to done
    // if let Err(err) = Todo::update_status(todo_id, true, &data.db).await {
        // Return an error response if updating fails
        // return HttpResponse::InternalServerError().body(format!("Failed to mark todo as done: {}", err));
    // }
    
    // Step 3: Return a success response
    // HttpResponse::Ok().body("Todo marked as done")
}

// Define an async function to handle marking a todo as undone
#[put("/mark_todo_undone")]
async fn mark_todo_undone(
    // Define the data type for the AppState
    // data: web::Data<AppState>,
    
    // Define the form type to extract the UpdateTodoStatus structure
    // form: web::Form<UpdateTodoStatus>
) -> HttpResponse {
    // Step 1: Extract the todo ID from the form
    // let todo_id = form.into_inner().id;
    
    // Step 2: Update the status of the todo to undone
    // if let Err(err) = Todo::update_status(todo_id, false, &data.db).await {
        // Return an error response if updating fails
        // return HttpResponse::InternalServerError().body(format!("Failed to mark todo as undone: {}", err));
    // }
    
    // Step 3: Return a success response
    // HttpResponse::Ok().body("Todo marked as undone")
}
