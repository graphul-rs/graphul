//! Example websocket application.
//!

use graphul::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::Methods,
    FileConfig, Graphul,
};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.static_file("/", "templates/index.html", FileConfig::default());

    app.get("/websocket", |ws: WebSocketUpgrade| async move {
        ws.on_upgrade(|mut socket: WebSocket| async move {
            let _ = socket
                .send(Message::Text("Hello Graphul".to_string()))
                .await;
        })
    });

    app.run("127.0.0.1:3000").await;
}
