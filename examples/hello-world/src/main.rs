use graphul::{http::Methods, Graphul, Context};

fn api_router() -> Graphul {
    let mut router = Graphul::router();

    router.get("/users/:id", |c: Context| async move {
        format!("User with id: {}", c.params("id"))
    });

    router
}

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/", || async { "Home" });

    app.add_router(api_router());

    app.run("127.0.0.1:8000").await;
}
