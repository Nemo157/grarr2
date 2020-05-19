#![feature(type_alias_impl_trait)]

use std::future::Future;
use http_service::HttpService;
use http_types::{Response, StatusCode, Request};

#[derive(Debug)]
struct App;

#[derive(Debug, Clone)]
struct State;

#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// test
    None,
}

type ConnectionFuture = impl Future<Output = Result<State, Error>>;
type ResponseFuture = impl Future<Output = Result<Response, Error>>;

impl HttpService for App {
    type Connection = State;
    type ConnectionError = Error;
    type ConnectionFuture = ConnectionFuture;
    type ResponseError = Error;
    type ResponseFuture = ResponseFuture;

    fn connect(&self) -> Self::ConnectionFuture {
        async move { Ok(State) }
    }

    fn respond(&self, state: Self::Connection, req: Request) -> Self::ResponseFuture {
        async move {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body("Hello, Alpaca!");
            Ok(res)
        }
    }
}

pub fn app() -> impl HttpService<ConnectionError = Error, ResponseError = Error> {
    App
}
