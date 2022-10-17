use graphul::{
    http::Methods,
    Context, Graphul, template::HtmlTemplate,
};
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/:name", |c: Context| async  move {
        let template = HelloTemplate { name: c.params("name") };
        HtmlTemplate(template)
    });

    app.run("127.0.0.1:8000").await;
}
