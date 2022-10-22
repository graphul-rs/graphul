use askama::Template;
use graphul::{http::Methods, template::HtmlTemplate, Context, Graphul};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/:name", |c: Context| async move {
        let template = HelloTemplate {
            name: c.params("name"),
        };
        HtmlTemplate(template)
    });

    app.run("127.0.0.1:8000").await;
}
