//! Example chat application.
//!

mod chat;

use chat::{domain, handlers};

use graphul::{http::Methods, FileConfig, Graphul};

#[tokio::main]
async fn main() {
    let state = domain::app_state();

    let mut app = Graphul::share_state(state);

    app.static_file("/", "templates/index.html", FileConfig::default());

    app.get("/websocket", handlers::websocket_handler);

    app.run("127.0.0.1:3000").await;
}
