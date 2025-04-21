use axum::{Router, routing::post};
use tokio::net::TcpListener;
use webhook_proxy::handler::webhook_handler;

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route("/proxy", post(webhook_handler));

    let listener = TcpListener::bind("127.0.0.1:3000").await;

    match listener {
        Ok(listener) => {
            match listener.local_addr() {
                Ok(addr) => println!("Socket is listening on {}", addr),
                Err(_) => println!("Local address failed"),
            }
            axum::serve(listener, app).await.unwrap();
        }
        Err(e) => {
            println!("Failed to bind to port 3000 {}", e);
        }
    }
}
