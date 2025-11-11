use axum::Router;

#[tokio::main]
async fn main() {
    let server_addr = "localhost:3100";
    let router = Router::new();

    println!("Launching back end: http://{server_addr}");
    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
