mod routes;
use tokio::net::TcpListener;

async fn main() {
    let app = router::create_routes();

    let listener = TcpListener::("127.0.0.1:3000").await.expect("Did not bind to port");
    println!("Server running");
    axum::serve(listener, app).await.expect("Did not start");

}
