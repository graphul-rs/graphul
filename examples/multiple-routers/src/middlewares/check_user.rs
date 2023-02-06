use graphul::{
    http::response::{Redirect, Response},
    middleware::Next,
    ContextPart, IntoResponse, Req,
};

pub async fn check_user_is_authenticated(ctx: ContextPart, request: Req, next: Next) -> Response {
    let login = ctx.query("login");
    if login == "false" || login.is_empty() {
        return Redirect::to("/login").into_response();
    }
    next.run(request).await
}
