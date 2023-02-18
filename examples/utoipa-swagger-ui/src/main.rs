mod routes;
mod swagger;

use std::sync::Arc;

use routes::todo::{self, Store};

use graphul::{http::Methods, Graphul};
use swagger::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let store = Arc::new(Store::default());
    let mut app = Graphul::share_state(store);

    let swagger = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi());
    app.merge(swagger);

    app.get("/todo", todo::list_todos);
    app.post("/todo", todo::create_todo);

    app.get("/todo/search", todo::search_todos);

    app.put("/todo/:id", todo::mark_done);
    app.delete("/todo/:id", todo::delete_todo);

    app.run("127.0.0.1:8000").await;
}
