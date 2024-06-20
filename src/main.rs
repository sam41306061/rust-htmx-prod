mod db;
mod leptos_axum;
mod utils;

use axum::{routing::get, Router};
use axum::{http::Request, Response};
use leptos::view;
use leptos_axum::LeptosHtml;
use tower_http::services::ServeDir;

async fn index() -> LeptosHtml {
    return view! {
<html lang="en">
    <head>
        <title>HTMX Is Neet!</title>
        <meta charset="UTF-8"></meta>
        <meta name="viewport" content="width=device-width, initial-scale=1"></meta>
        <link href="/assets/index.css" rel="stylesheet"></link>
        <script src="https://unpkg.com/htmx.org@2.0.0/dist/htmx.js" integrity="sha384-Xh+GLLi0SMFPwtHQjT72aPG19QvKB8grnyRbYBNIdHWc2NkCrz65jlU7YrzO6qRp" crossorigin="anonymous"></script>
        <script src="/assets/bundle.js"></script>
    </head>
    <body>
        // <div class="bg-green-100 text-blue-800 w-full h-full">hello, mom</div>
        <div class="bg-green-100 text-blue-800 w-full h-full" hx-target="this" hx-swap="outerHTML">
        <div id="counting"> 0 </div>
        <button hx-post="/clicked"
        hx-trigger="click"
        hx-target="#counting">
        Click To Edit
        </button>
    </div>
    </body>
</html>
    }.into();
}

async fn clicked(req:Request<()>) -> Response {
    return Response::Ok()
    .content_type("text/plain")
    .body("You clicked me!!")
}


#[tokio::main]
async fn main() {
    env_logger::init();


    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", ServeDir::new("dist"));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
