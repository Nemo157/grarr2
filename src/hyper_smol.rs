use futures_core::stream::TryStream;
use futures_io::AsyncRead;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt as _};

#[derive(Debug, Clone)]
pub struct SmolExecutor;

impl<F: Future<Output = ()> + Send + Sync + 'static> hyper::rt::Executor<F> for SmolExecutor {
    fn execute(&self, future: F) {
        smol::Task::spawn(future).detach();
    }
}

pin_project_lite::pin_project! {
    pub struct WrapIncoming<I> {
        #[pin]
        incoming: I,
    }
}

impl<T> WrapIncoming<T> {
    pub fn new(incoming: T) -> Self {
        Self { incoming }
    }
}

impl<I: TryStream> hyper::server::accept::Accept for WrapIncoming<I>
where
    I::Ok: AsyncRead,
{
    type Conn = Compat<I::Ok>;
    type Error = I::Error;

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        self.project()
            .incoming
            .try_poll_next(cx)
            .map(|opt| opt.map(|res| res.map(|stream| stream.compat())))
    }
}
