use graphul::{
    http::{response::Redirect, Methods},
    Context, Graphul, IntoResponse,
};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    // http://127.0.0.1:8000?redirect=true
    app.get("/", |c: Context| async move {
        // Redirect::temporary(uri) Create a new Redirect that uses a 307 Temporary Redirect status code.
        // Redirect::permanent(uri) Create a new Redirect that uses a 308 Permanent Redirect status code.
        if c.query("redirect") == "true" {
            return Redirect::to("/hi/samuel").into_response(); // Create a new Redirect that uses a 303 See Other status code.
        }
        "index".into_response()
    });

    app.get("/hi/:name", |c: Context| async move {
        format!("hello, {}", c.params("name"))
    });

    app.run("127.0.0.1:8000").await;
}
