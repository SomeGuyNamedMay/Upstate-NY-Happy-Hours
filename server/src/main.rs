// ./backend/src/main.rs
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // 1. Create a router for your API routes
    let api_routes = Router::new().route("/hello", get(|| async { "Hello from Axum API!" }));

    // 2. Serve the Yew 'dist' folder and fall back to index.html for SPA routing
    let frontend_service = ServeDir::new("../client/dist")
        .not_found_service(ServeDir::new("../client/dist/index.html"));

    // 3. Combine your API and frontend routes
    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(frontend_service);

    // 4. Bind and run the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}
