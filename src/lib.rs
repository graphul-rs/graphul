//pub mod types;

//use types::*;

mod color;
mod listen;
mod app;
pub mod http;

use std::net::SocketAddr;

pub use axum::extract::Json;
use axum::handler::Handler;
pub use axum::response::IntoResponse;
use axum::routing::{delete, get, head, options, patch, post, put, trace};
use axum::Router;
use http::resource::Resource;

pub type Body = axum::body::Body;

pub type Request = axum::http::Request<Body>;

pub struct Group<'a> {
    app: &'a mut RustFul,
    prefix: String,
}

impl<'a> Group<'a> {
    pub fn resource(&mut self, path: &str, res: impl Resource + 'static) {
        let route_name = self.get_route_name(path);
        self.app.resource(route_name.as_str(), res);
    }
}

impl<'a> http::Methods for Group<'a> {
    fn post<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.post(route_name.as_str(), handler);
    }

    fn get<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.get(route_name.as_str(), handler);
    }
    fn put<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.put(route_name.as_str(), handler);
    }

    fn delete<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.delete(route_name.as_str(), handler);
    }
    fn head<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.head(route_name.as_str(), handler);
    }

    fn options<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.options(route_name.as_str(), handler);
    }
    fn patch<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.patch(route_name.as_str(), handler);
    }

    fn trace<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        let route_name = self.get_route_name(path);
        self.app.trace(route_name.as_str(), handler);
    }
}

impl<'a> Group<'a> {
    fn new(app: &'a mut RustFul, prefix: &str) -> Self {
        Group {
            app: app,
            prefix: prefix.to_string()
        }
    }

    fn get_route_name(&self, name: &str) -> String {
        if name == "/" {
            return self.prefix.clone();
        }
        format!("{}{}", self.prefix, name)
    }

    pub fn group(&mut self, name: &str) -> Group {
        self.app
            .group(format!("/{}/{}", self.prefix, name).as_str())
    }
}

pub struct RustFul {
    routes: Router<Body>,
    count_routes: usize,
}

impl RustFul {
    pub fn resource<T: Resource + 'static>(&mut self, path: &str, _res: T) {
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

impl http::Methods for RustFul {
    fn get<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, get(handler));
    }
    fn post<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, post(handler));
    }
    fn put<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, put(handler));
    }
    fn delete<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, delete(handler));
    }
    fn head<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, head(handler));
    }
    fn options<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(path, options(handler));
    }
    fn patch<T, H>(&mut self, _path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(_path, patch(handler));
    }
    fn trace<T, H>(&mut self, _path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static,
    {
        self.increase_route_counter();
        self.routes = self.routes.clone().route(_path, trace(handler));
    }
}

impl RustFul {
    pub fn new() -> Self {
        Self {
            routes: Router::new(),
            count_routes: 0,
        }
    }

    fn increase_route_counter(&mut self) {
        self.count_routes += 1;
    }

    pub fn group(&mut self, name: &str) -> Group {
        Group::new(self, format!("/{}", name).as_str())
    }

    pub async fn run(self, addr: &str) {

        let addr: SocketAddr = addr.parse().unwrap();

        listen::startup_message(&addr, false, self.count_routes);

        axum::Server::bind(&addr)
            .serve(self.routes.into_make_service())
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
