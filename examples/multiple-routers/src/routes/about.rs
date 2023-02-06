use graphul::{http::Methods, Graphul};

async fn about() -> &'static str {
    "About this page ..."
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    router.get("/about", about);

    router
}
