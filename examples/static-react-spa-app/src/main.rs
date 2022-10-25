use graphul::{http::Methods, Context, FolderConfig, Graphul};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.static_files("/", "app/build", FolderConfig::spa());

    app.get("/api/:name", |c: Context| async move {
        format!("Hello, {}", c.params("name"))
    });

    app.run("127.0.0.1:8000").await;
}
