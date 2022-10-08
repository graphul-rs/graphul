use async_trait::async_trait;
use hyper::StatusCode;

use super::context::Context;
use super::response::Response;

/*

take from : https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html
async in Traits
Currently, async fn cannot be used in traits. The reasons for this are somewhat complex, but there are plans to remove this restriction in the future.

In the meantime, however, this can be worked around using the async-trait crate from crates.io.

Note that using these trait methods will result in a heap allocation per-function-call.
This is not a significant cost for the vast majority of applications, but should be considered
when deciding whether to use this functionality in the public API of a low-level function that is
expected to be called millions of times a second.

*/

#[async_trait]
pub trait Resource {
    async fn get(&self, mut ctx: Context) -> Response {
        ctx.response
            .status(StatusCode::NOT_IMPLEMENTED)
            .send("Method Not Allowed")
    }
    async fn post(&self, mut ctx: Context) -> Response {
        ctx.response
            .status(StatusCode::NOT_IMPLEMENTED)
            .send("Method Not Allowed")
    }
    async fn put(&self, mut ctx: Context) -> Response {
        ctx.response
            .status(StatusCode::NOT_IMPLEMENTED)
            .send("Method Not Allowed")
    }
    async fn delete(&self, mut ctx: Context) -> Response {
        ctx.response
            .status(StatusCode::NOT_IMPLEMENTED)
            .send("Method Not Allowed")
    }
    async fn path(&self, mut ctx: Context) -> Response {
        ctx.response
            .status(StatusCode::NOT_IMPLEMENTED)
            .send("Method Not Allowed")
    }
}
