use graphul::{http::Methods, middleware, Graphul};

use crate::middlewares::check_user::check_user_is_authenticated;

async fn admin() -> &'static str {
    "Protected Admin Route"
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    // http://127.0.0.1:8000/admin?login=true
    router.get("/admin", admin);
    router.middleware(middleware::from_fn(check_user_is_authenticated));

    router
}
