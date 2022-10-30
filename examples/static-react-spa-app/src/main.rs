mod data;

use data::articles;
use graphul::{
    http::{utils::Method, Methods, StatusCode},
    middleware::tower::cors,
    Context, FolderConfig, Graphul, IntoResponse,
};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.static_files("/", "app/build", FolderConfig::spa());

    app.get(
        "/api/articles",
        |c: Context| async move { c.json(articles()) },
    );

    app.get("/api/article/:id", |c: Context| async move {
        let id = match c.params("id").parse::<usize>() {
            Ok(id) => id - 1,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Id is not a number").into_response();
            }
        };
        match articles().get(id) {
            Some(item) => c.json(item.clone()).into_response(),
            None => (StatusCode::NOT_FOUND, "article does not exist :(").into_response(),
        }
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
