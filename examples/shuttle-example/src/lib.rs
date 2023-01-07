use graphul::{http::Methods, Graphul};
use sync_wrapper::SyncWrapper;

#[shuttle_service::main]
async fn graphul() -> shuttle_service::ShuttleAxum {
    let mut app = Graphul::new();

    app.get("/", || async { "Hello, World 👋!" });
    let sync_wrapper = SyncWrapper::new(app.export_routes());

    Ok(sync_wrapper)
}
