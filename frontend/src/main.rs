use axum::Router;

#[tokio::main]
async fn main() {
    let server_addr = "localhost:3000";
    let router = Router::new();

    println!("Launching front end: http://{server_addr:?}");
    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
