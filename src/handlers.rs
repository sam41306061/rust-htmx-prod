use actix_web::{post, web, HttpResponse};
use crate::models::{Todo, AppState};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodo {
    description: String,
}

struct UpdateTodoStatus {
    id: i32,
}

#[post("/add_todo")]
async fn add_todo(
    data: web::Data<AppState>,
    form: web::Form<CreateTodo>
) -> HttpResponse {
    // Step 1: Extract the todo description from the form
    // Step 2: Create a new Todo object
    // Step 3: Save the Todo object to the database
    // Step 4: Return a success response
}

#[get("/get_todos")]
async fn get_todos(
    data: web::Data<AppState>
) -> HttpResponse {
    // Step 1: Fetch todos from the database
    // Step 2: Check for errors and return the appropriate response
}

#[put("/mark_todo_done")]
async fn mark_todo_done(
    data: web::Data<AppState>,
    form: web::Form<UpdateTodoStatus>
) -> HttpResponse {
    // Step 1: Extract the todo ID from the form
    // Step 2: Update the status of the todo to done
    // Step 3: Return a success response
}

#[put("/mark_todo_undone")]
async fn mark_todo_undone(
    data: web::Data<AppState>,
    form: web::Form<UpdateTodoStatus>
) -> HttpResponse {
    // Step 1: Extract the todo ID from the form
    // Step 2: Update the status of the todo to undone
    // Step 3: Return a success response
}
#[get("/categories")]
async fn get_categories(
    data: web::Data<AppState>
) -> HttpResponse {
    // Step 1: Fetch categories from the database
    // Step 2: Check for errors and return the appropriate response
}

#[get("/category_todo_count")]
async fn get_category_todo_count(
    data: web::Data<AppState>,
    form: web::Query<UpdateTodoStatus>
) -> HttpResponse {
    // Step 1: Fetch the todo count for the given category
    // Step 2: Check for errors and return the appropriate response
}
