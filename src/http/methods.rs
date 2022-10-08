use axum::handler::Handler;

use crate::Body;

pub trait Methods {
    fn post<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static;

    fn put<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static;

    fn get<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static;

    fn delete<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static;

    fn patch<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static;

    fn options<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static;

    fn trace<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static;

    fn head<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, Body>,
        T: 'static;
}
