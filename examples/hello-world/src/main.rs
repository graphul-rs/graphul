use graphul::{http::Methods, Graphul};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/", || async { "Hello, World 👋!" });

    app.run("127.0.0.1:8000").await;
}
