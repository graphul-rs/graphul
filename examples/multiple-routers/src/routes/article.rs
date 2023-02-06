use graphul::{http::Methods, Context, Graphul};

async fn article() -> &'static str {
    "Article list"
}

async fn get_article(ctx: Context) -> String {
    format!("Article id: {}", ctx.params("id"))
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut article_group = router.group("article");

    // http://127.0.0.1:8000/article
    article_group.post("/", article);

    // http://127.0.0.1:8000/article/my_id
    article_group.get("/:id", get_article);

    router
}
