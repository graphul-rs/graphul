mod about;
mod admin;
mod article;

use graphul::Graphul;

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    router.add_routers(vec![
        about::routes().await,
        article::routes().await,
        admin::routes().await,
    ]);

    router
}
