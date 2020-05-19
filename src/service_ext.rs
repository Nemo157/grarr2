use std::task::{Context, Poll};
use tower_service::Service;

pub struct Map<S, F> {
    inner: S,
    func: F,
}

impl<T, U, S: Service<U>, F: FnMut(T) -> U> Service<T> for Map<S, F> {
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: T) -> Self::Future {
        self.inner.call((&mut self.func)(request))
    }
}

pub trait ServiceExt<T>: Service<T> {
    fn map_request<U, F: FnMut(U) -> T>(self, func: F) -> Map<Self, F>
    where
        Self: Sized,
    {
        Map { inner: self, func }
    }
}

impl<T, S: Service<T>> ServiceExt<T> for S {}
