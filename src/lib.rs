//pub mod types;

//use types::*;

mod app;
mod color;
mod fs;
pub mod http;
mod listen;
pub mod middleware;
pub mod template;

use std::convert::Infallible;
use std::io;
use std::net::SocketAddr;

pub use async_trait::async_trait;

pub use axum::extract;
use axum::handler::Handler;
pub use axum::response::IntoResponse;
use axum::routing::{delete, get, get_service, head, options, patch, post, put, trace, Route};
use axum::Router;

pub use http::request::Context;
pub use http::request::ContextPart;
use http::resource::Resource;
use http::StatusCode;
use hyper::service::Service;
use hyper::Request;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_status::SetStatus;
use tower_layer::Layer;

use tower::ServiceExt;

pub type FolderConfig = fs::FolderConfig;
pub type FileConfig = fs::FileConfig;

pub type Body = axum::body::Body;

pub type Req = axum::http::Request<Body>;

pub struct Group<'a, S = ()> {
    app: &'a mut Graphul<S>,
    prefix: String,
}

impl<'a, S> Group<'a, S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn resource(&mut self, path: &str, res: impl Resource<S> + 'static) {
        let route_name = self.get_route_name(path);
        self.app.resource(route_name.as_str(), res);
    }
}

impl<'a, S> http::Methods<S, Body> for Group<'a, S>
where
    S: Clone + Send + Sync + 'static,
{
    fn post<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.post(route_name.as_str(), handler);
    }

    fn get<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.get(route_name.as_str(), handler);
    }
    fn put<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.put(route_name.as_str(), handler);
    }

    fn delete<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.delete(route_name.as_str(), handler);
    }
    fn head<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.head(route_name.as_str(), handler);
    }

    fn options<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.options(route_name.as_str(), handler);
    }
    fn patch<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.patch(route_name.as_str(), handler);
    }

    fn trace<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.trace(route_name.as_str(), handler);
    }
}

impl<'a, S> Group<'a, S>
where
    S: Clone + Send + Sync + 'static,
{
    fn new(app: &'a mut Graphul<S>, prefix: &str) -> Self {
        Group {
            app,
            prefix: prefix.to_string(),
        }
    }

    fn get_route_name(&self, name: &str) -> String {
        if name == "/" {
            return self.prefix.clone();
        }
        format!("{}{}", self.prefix, name)
    }

    pub fn group(&mut self, name: &str) -> Group<S> {
        self.app
            .group(format!("/{}/{}", self.prefix, name).as_str())
    }
}

pub struct Graphul<S = ()> {
    routes: Router<S, Body>,
    count_routes: usize,
    state: S,
}

impl<S> Graphul<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn resource<T: Resource<S> + 'static>(&mut self, path: &str, _res: T) {
        // get
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, get(T::get));

        // post
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, post(T::post));

        // put
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, put(T::put));

        // delete
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, delete(T::delete));

        // patch
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, patch(T::patch));

        // options
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, options(T::options));

        // trace
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, trace(T::trace));

        // head
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, head(T::head));
    }
}

impl<S> http::Methods<S, Body> for Graphul<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn get<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, get(handler));
    }
    fn post<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, post(handler));
    }
    fn put<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, put(handler));
    }
    fn delete<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, delete(handler));
    }
    fn head<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, head(handler));
    }
    fn options<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, options(handler));
    }
    fn patch<T, H>(&mut self, _path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(_path, patch(handler));
    }
    fn trace<T, H>(&mut self, _path: &str, handler: H)
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(_path, trace(handler));
    }
}

impl Graphul<()> {
    pub fn new() -> Self {
        Self {
            routes: Router::new(),
            count_routes: 0,
            state: (),
        }
    }

    // new alias to create sub-routes
    pub fn router() -> Self {
        Self {
            routes: Router::new(),
            count_routes: 0,
            state: (),
        }
    }

    #[warn(dead_code)]
    fn get() -> Self {
        // v 0.6
        todo!()
    }

    #[warn(dead_code)]
    fn post() -> Self {
        // v 0.6
        todo!()
    }

    #[warn(dead_code)]
    fn put() -> Self {
        // v 0.6
        todo!()
    }

    #[warn(dead_code)]
    fn delete() -> Self {
        // v 0.6
        todo!()
    }

    #[warn(dead_code)]
    fn patch() -> Self {
        // v 0.6
        todo!()
    }

    #[warn(dead_code)]
    fn options() -> Self {
        // v 0.6
        todo!()
    }

    #[warn(dead_code)]
    fn trace() -> Self {
        // v 0.6
        todo!()
    }

    #[warn(dead_code)]
    fn head() -> Self {
        // v 0.6
        todo!()
    }
}

impl Default for Graphul<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Graphul<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn share_state(state: S) -> Self {
        Self {
            routes: Router::new(),
            count_routes: 0,
            state,
        }
    }

    fn increase_route_counter(&mut self) {
        self.count_routes += 1;
    }

    pub fn add_router(&mut self, route: Graphul<S>) {
        self.routes = self.routes.clone().merge(route.routes);
        self.count_routes += route.count_routes
    }

    pub fn add_routers(&mut self, routes: Vec<Graphul<S>>) {
        for route in routes {
            self.routes = self.routes.clone().merge(route.routes);
            self.count_routes += route.count_routes
        }
    }

    pub fn set_server_file_config(
        &self,
        file_dir: String,
        compress: bool,
        chunk_size: Option<usize>,
    ) -> ServeFile {
        let mut serve_file = ServeFile::new(file_dir);
        if compress {
            serve_file = serve_file
                .precompressed_gzip()
                .precompressed_deflate()
                .precompressed_br()
        }
        if let Some(chunk_size) = chunk_size {
            serve_file = serve_file.with_buf_chunk_size(chunk_size)
        }
        serve_file
    }

    pub fn static_files(&mut self, path: &'static str, dir: &'static str, config: FolderConfig) {
        let mut serve_dir: ServeDir<SetStatus<ServeFile>>;
        if config.spa {
            serve_dir = ServeDir::new(dir)
                .append_index_html_on_directories(config.index)
                .fallback(SetStatus::new(
                    self.set_server_file_config(
                        format!("{}/index.html", dir),
                        config.compress,
                        config.chunk_size,
                    ),
                    StatusCode::OK,
                ));
        } else {
            let mut not_found = "";
            if let Some(not_f) = config.not_found {
                not_found = not_f;
            }
            serve_dir = ServeDir::new(dir)
                .append_index_html_on_directories(config.index)
                .fallback(SetStatus::new(
                    self.set_server_file_config(
                        not_found.to_string(),
                        config.compress,
                        config.chunk_size,
                    ),
                    StatusCode::NOT_FOUND,
                ));
        }
        if config.compress {
            serve_dir = serve_dir
                .precompressed_gzip()
                .precompressed_deflate()
                .precompressed_br()
        }
        if let Some(chunk_size) = config.chunk_size {
            serve_dir = serve_dir.with_buf_chunk_size(chunk_size)
        }
        let serve_dir = get_service(serve_dir).handle_error(Graphul::<S>::handle_error);
        self.routes = self.routes.clone().nest_service(path, serve_dir);
    }

    pub fn static_file(&mut self, path: &'static str, file: &'static str, config: FileConfig) {
        let serve_dir =
            self.set_server_file_config(file.to_string(), config.compress, config.chunk_size);
        self.routes = self.routes.clone().route(
            path,
            get(move |request: Request<Body>| async move {
                let serve_dir = get_service(serve_dir).handle_error(Graphul::<S>::handle_error);
                serve_dir.oneshot(request).await
            }),
        );
    }

    async fn handle_error(_err: io::Error) -> impl IntoResponse {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
    }

    pub fn middleware<L>(&mut self, service: L)
    where
        L: Layer<Route<Body>> + Clone + Send + 'static,
        L::Service: Service<Request<Body>> + Clone + Send + 'static,
        <L::Service as Service<Request<Body>>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request<Body>>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request<Body>>>::Future: Send + 'static,
    {
        self.routes = self.routes.clone().route_layer(service);
    }

    pub fn group(&mut self, name: &str) -> Group<S> {
        Group::new(self, format!("/{}", name).as_str())
    }

    async fn fallback(req: Req) -> (StatusCode, String) {
        (
            StatusCode::NOT_FOUND,
            format!("Cannot {} {}", req.method().as_str(), req.uri()),
        )
    }

    pub fn export_routes(self) -> Router {
        self.routes
            .with_state(self.state)
            .fallback(Graphul::<S>::fallback)
    }

    pub async fn run(self, addr: &str) {
        let addr: SocketAddr = addr.parse().unwrap();

        listen::startup_message(&addr, false, self.count_routes);

        axum::Server::bind(&addr)
            .serve(
                self.routes
                    .with_state(self.state)
                    .fallback(Graphul::<S>::fallback)
                    .into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .unwrap();
    }
}
