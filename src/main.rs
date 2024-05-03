#![warn(clippy::all)]

use rush::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new().run().await.expect("Server crashed due to internal error");
}
