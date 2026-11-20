use axum::{Router, routing::get, extract::path, response::html,}; 

// Could use tower, will investiagte

pub fn create_routes() -> Router { 
    Router::new()
        .route("/", get(pages::index))
        .route("/auth", get(pages::auth))
        .route("/users/{id}", get(pages::users))

}