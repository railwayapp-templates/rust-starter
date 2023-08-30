// This starter uses the `axum` crate to create an asyncrohnous web server
// The async runtime being used, is `tokio`
// This starter also has logging, powered by `tracing` and `tracing-subscriber`

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;

// This derive macro allows our main function to run asyncrohnous code. Without it, the main function would run syncrohnously
#[tokio::main]
async fn main() {
    // First, we initialize the tracing subscriber with default configuration
    // This is what allows us to print things to the console
    tracing_subscriber::fmt::init();

    // Then, we create a router, which is a way of routing requests to different handlers
    let app = Router::new()
        // In order to add a route, we use the `route` method on the router
        // The `route` method takes a path (as a &str), and a handler (MethodRouter)
        // In our invocation below, we create a route, that goes to "/"
        // We specify what HTTP method we want to accept on the route (via the `get` function)
        // And finally, we provide our route handler
        // The code of the root function is below
        .route("/", get(root))
        // This can be repeated as many times as you want to create more routes
        // We are also going to create a more complex route, using `impl IntoResponse`
        // The code of the complex function is below
        .route("/complex", get(complex));

    // Next, we need to run our app with `hyper`, which is the HTTP server used by `axum`
    // We need to create a `SocketAddr` to run our server on
    // Before we can create that, we need to get the port we wish to serve on
    // This code attempts to get the port from the environment variable `PORT`
    // If it fails to get the port, it will default to "3000"
    // We then parse the `String` into a `u16`, to which if it fails, we panic
    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".into())
        .parse()
        .expect("failed to convert to number");
    // We then create a socket address, listening on 0.0.0.0:PORT
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    // We then log the address we are listening on, using the `info!` macro
    // The info macro is provided by `tracing`, and allows us to log stuff at an info log level
    tracing::info!("listening on {}", addr);
    // Then, we run the server, using the `bind` method on `Server`
    // `axum::Server` is a re-export of `hyper::Server`
    axum::Server::bind(&addr)
        // We then convert our Router into a `Service`, provided by `tower`
        .serve(app.into_make_service())
        // This function is async, so we need to await it
        .await
        // Then, we unwrap the result, to which if it fails, we panic
        .unwrap();
}

// This is our route handler, for the route root
// Make sure the function is `async`
// We specify our return type, `&'static str`, however a route handler can return anything that implements `IntoResponse`

async fn root() -> &'static str {
    "Hello, World!"
}

// This is our route handler, for the route complex
// Make sure the function is async
// We specify our return type, this time using `impl IntoResponse`

async fn complex() -> impl IntoResponse {
    // For this route, we are going to return a Json response
    // We create a tuple, with the first parameter being a `StatusCode`
    // Our second parameter, is the response body, which in this example is a `Json` instance
    // We construct data for the `Json` struct using the `serde_json::json!` macro
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Hello, World!"
        })),
    )
}
