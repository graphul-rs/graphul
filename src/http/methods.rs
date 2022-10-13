use axum::handler::Handler;

pub trait Methods<S, B> {
    fn post<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S, B>,
        T: 'static;

    fn put<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S, B>,
        T: 'static;

    fn get<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S, B>,
        T: 'static;

    fn delete<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S, B>,
        T: 'static;

    fn patch<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S, B>,
        T: 'static;

    fn options<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S, B>,
        T: 'static;

    fn trace<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S, B>,
        T: 'static;

    fn head<T, H>(&mut self, path: &str, handler: H)
    where
        H: Handler<T, S, B>,
        T: 'static;
}
