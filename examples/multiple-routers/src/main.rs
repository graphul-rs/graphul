mod middlewares;
mod routes;

use graphul::{http::Methods, Graphul};
use routes::routes;

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/login", || async { "Login!" });

    app.add_router(routes().await);

    // print routes on the console
    // it will print :
    //    ["GET /login", "GET /about", "POST /article", "GET /article/:id", "GET /admin"]
    println!("{:?}", app.routes());

    app.run("127.0.0.1:8000").await;
}
