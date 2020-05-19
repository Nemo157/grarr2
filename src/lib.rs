#![feature(type_alias_impl_trait, never_type, associated_type_bounds)]

use http::StatusCode;
use std::{
    future::Future,
    net::SocketAddr,
    task::{Context, Poll},
};
use tower_service::Service;

#[derive(Debug, Clone)]
pub struct App;

#[derive(Debug)]
pub struct Connection {
    peer: SocketAddr,
    app: App,
}

#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// test
    None,
}

pub type Infallible<T> = Result<T, !>;

pub type ConnectionFuture = impl Future<Output = Infallible<Connection>> + Send + Sync;

impl Service<SocketAddr> for App {
    type Response = Connection;
    type Error = !;
    type Future = ConnectionFuture;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Infallible<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, peer: SocketAddr) -> Self::Future {
        let conn = Connection {
            peer,
            app: self.clone(),
        };
        async move { Ok(conn) }
    }
}

pub type Request = http::Request<hyper::Body>;
pub type Response = http::Response<hyper::Body>;
pub type ResponseFuture = impl Future<Output = Infallible<Response>> + Send + Sync;

impl Service<Request> for Connection {
    type Response = Response;
    type Error = !;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Infallible<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Request) -> Self::Future {
        async move {
            let res = http::Response::builder()
                .status(StatusCode::OK)
                .body("Hello, Alpaca!".into())
                .unwrap();
            Ok(res)
        }
    }
}

pub fn app() -> App {
    App
}
