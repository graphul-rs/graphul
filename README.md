<p align="center">
  <a href="https://github.com/rustful-rs/rustful">
    <img alt="Fiber" height="200" src="./img/logo.png">
  </a>
  <br>
</p>

<p>
<b>Graphul</b> is an <a href="https://github.com/expressjs/express">Express</a> inspired <b>web framework</b> using a powerful extractor system built on top of <a href="https://tokio.rs/">Tokio, Tower, Axum, and Hyper</a>. Designed to improve, speed, and scale your microservices with a friendly syntax, Graphul is built with <a href="https://www.rust-lang.org/">Rust</a>. that means Graphul gets memory safety, reliability, concurrency, and performance for free. helping to save money on infrastructure.
</p>

## ⚡️ Quickstart

```rust
use graphul::{Graphul, http::Methods};


#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/", || async {
        "Hello, World 👋!"
    });

    app.run("127.0.0.1:8000").await;
}
```

## Context

```rust
use graphul::{Graphul, Context, http::Methods };


#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    // /samuel?country=Colombia
    app.get("/:name", |c: Context| async move {

       let name = c.params("name");
       let country = c.query("country");

        format!("My name is {}, I'm from {}", name, country)
    });

    app.run("127.0.0.1:8000").await;

}
```

## JSON

```rust
use graphul::{Json, Graphul, http::Methods};
use serde_json::json;


#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/", || async {
        Json(json!({
            "name": "full_name",
            "age": 98,
            "phones": [
                format!("+44 {}", 8)
            ]
        }))
    });

    app.run("127.0.0.1:8000").await;
}
```

## Resource

```rust
use graphul::{Json, Graphul, http::{StatusCode, resource::Resource, response::Response}, Request, IntoResponse};
use async_trait::async_trait;
use serde_json::json;

type ResValue = HashMap<String, String>;

struct Article;

#[async_trait]
impl Resource for Article {

    async fn get(_req: Request) -> Response {
        let posts = json!({
            "posts": ["Article 1", "Article 2", "Article ..."]
        });
        (StatusCode::OK, Json(posts)).into_response()
    }

    async fn post(_req: Request) -> Response {
        async fn post(ctx: Context<AppState>) -> Response {
        // you can use ctx.parse_params() or ctx.parse_query()
        let value: Json<ResValue> = match ctx.payload().await {
            Ok(data) => data,
            Err(err) => return err.into_response(),
        };

        (StatusCode::CREATED, value).into_response()
    }

    // you can use put, delete, head, patch and trace
}



#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.resource("/article", Article);

    app.run("127.0.0.1:8000").await;

}
```

## Groups


```rust
use graphul::{
    Json,
    extract::Path,
    Graphul,
    http::{ Methods, StatusCode }
};

use serde_json::json;


async fn index() -> &'static str {
    "index handler"
}

async fn name(Path(name): Path<String>) -> impl IntoResponse {
    let user = json!({
        "response": format!("my name is {}", name)
    });
    (StatusCode::CREATED, Json(user))
}

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    // GROUP /api
    let mut api = app.group("api");

    // GROUP /api/user
    let mut user = api.group("user");

    // GET POST PUT DELETE ... /api/user
    user.resource("/", Article)

    // GET /api/user/samuel
    user.get("/:name", name);

    // GROUP /api/post
    let mut post = api.group("post");

    // GET /api/post
    post.get("/", index);

    // GET /api/post/all
    post.get("/all", || async move {
        Json(json!({"message": "hello world!"}))
    });

    app.run("127.0.0.1:8000").await;
}
```

## Share state

```rust
use graphul::{http::Methods, extract::State, Graphul};

#[tokio::main]
async fn main() {
    #[derive(Clone)]
    struct AppState {
        data: String
    }

    let state = AppState { data: "Hello, World 👋!".to_string() };
    let mut app = Graphul::share_state(state);

    app.get("/", |State(state): State<AppState>| async {
        state.data
    }); // .middelware();

    app.run("127.0.0.1:8000").await;
}
```

## Share state with Resource

```rust
use async_trait::async_trait;
use graphul::{
    http::{resource::Resource, response::Response, StatusCode},
    Context, Graphul, IntoResponse,
};
use serde_json::json;

struct Article;

#[derive(Clone)]
struct AppState {
    data: Vec<&'static str>,
}

#[async_trait]
impl Resource<AppState> for Article {

    async fn get(ctx: Context<AppState>) -> Response {
        let article = ctx.state();

        let posts = json!({
            "posts": article.data,
        });
        (StatusCode::OK, ctx.json(posts)).into_response()
    }

    // you can use post, put, delete, head, patch and trace
}

#[tokio::main]
async fn main() {
    let state = AppState {
        data: vec!["Article 1", "Article 2", "Article 3"],
    };
    let mut app = Graphul::share_state(state);

    app.resource("/article", Article);


    app.run("127.0.0.1:8000").await;
}
```

## Middleware

```rust
use graphul::{
    Next,
    Req,
    middleware,
    http::{response::Response,Methods},
    Graphul
};

async fn my_middleware( request: Req, next: Next ) -> Response {

    // your logic

    next.run(request).await
}

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/", || async {
        "hello world!"
    });
    app.middleware(middleware::from_fn(my_middleware));

    app.run("127.0.0.1:8000").await;
}
```
