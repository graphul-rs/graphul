mod data;

use data::get_data;
use graphul::{
    http::{utils::Method, Methods},
    middleware::tower::cors,
    Context, FolderConfig, Graphul,
};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.static_files("/", "app/build", FolderConfig::spa());

    app.get("/api/:name", |c: Context| async move {
        c.json(get_data(c.params("name")))
    });

    app.middleware(
        cors::CorsLayer::new()
            // allow `GET` and `POST` when accessing the resource
            .allow_methods([Method::GET, Method::POST])
            // allow requests from any origin
            .allow_origin(cors::Any),
    );

    app.run("127.0.0.1:8000").await;
}
