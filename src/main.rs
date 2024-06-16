// Import necessary libraries and modules

use std::{env, io};

use actix_files::Files;
use actix_web::{web::Data, get, App, HttpServer, HttpResponse, Responder};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct HomePage {
    adjective: &'static str,
    verb: &'static str,
}

#[get("/")]
async fn homepage(hb: Data<Handlebars<'_>>) -> impl Responder {
    let homepage = HomePage {
        adjective: "most stellar",
        verb: "known and/or dreamed of",
    };
    let html = hb.render("index", &homepage).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let address = env::var("BIND_ADDRESS")
        .unwrap_or_else(|_err| "localhost:8080".to_string());

    let template_service = {
        let mut handlebars = Handlebars::new();

        handlebars
            .register_templates_directory(".html", "web/templates")
            .unwrap();

        Data::new(handlebars)
    };

    let server = move || App::new()
        .app_data(template_service.clone())
        .service(Files::new("/public", "web/public").show_files_listing())
        .service(homepage);

    HttpServer::new(server)
        .bind(address)?
        .run()
        .await
}
    // Step 1: Initialize the HttpServer with routes
    // HttpServer::new(|| {
        // App::new()
            // .service(handlers::add_todo)
            // .service(handlers::get_todos)
            // .service(handlers::mark_todo_done)
            // .service(handlers::mark_todo_undone)
    // })

    // Step 2: Bind the server to the address and run it
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await