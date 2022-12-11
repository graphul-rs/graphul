use graphul::{
    extract::{DefaultBodyLimit, Multipart},
    http::response::Html,
    http::Methods,
    ContextPart, Graphul,
};

#[tokio::main]
async fn main() {
    // build our application with some routes
    let mut app = Graphul::new();
    app.get("/", show_form);
    app.post("/", accept_form);

    // limit the size of the file
    app.middleware(DefaultBodyLimit::max(250 * 1024 * 1024 /* 250mb */));

    app.run("0.0.0.0:3000").await;
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn accept_form(ctx: ContextPart, mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        // POST /?name=my_new_file_name
        let mut file_name = ctx.query("name");
        if file_name.is_empty() {
            file_name = field.file_name().unwrap().to_string();
        }

        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{}` (`{}`: `{}`) is {} bytes",
            name,
            file_name,
            content_type,
            data.len()
        );
    }
}
