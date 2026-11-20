use axum::{Router, routing::get, extract::path, response::html, }; 
use crate::handler:pages;
// Could use tower, will investiagte

pub fn create_routes() -> Router { 
    Router::new()
        // Base
        .route("/", get(pages::index))
        .route("/users/{id}", get(pages::users))
        .route("/admin_dash", get(pages::admin_dashboard).post(pages::admin_alterations))
        .route("/listings", get(pages::listings))
        .route("/listing/{id}", get(pages::listing))
        .route("/contact_us", get(pages::contact_us))

        // auth
        .route("/auth", get(pages::login_page).post(pages::login))
        .route("/verify_email", get(pages::email_verification_page).post(pages::verify_email))
        .route("/verification_success", get(pages::verification_success))

        // Payment
        .route("/payment/{id}", get(pages::payment_page).post(pages::verify_payment))
        .route("/payment_success/{id}", get(pages::payment_success))
        
}
