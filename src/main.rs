mod hyper_smol;
mod service_ext;

use hyper_smol::{SmolExecutor, WrapIncoming};
use service_ext::ServiceExt as _;
use tokio_util::compat::Compat;

fn main() -> anyhow::Result<()> {
    smol::run(async {
        let listener = smol::Async::<std::net::TcpListener>::bind("127.0.0.1:8080")?;
        let incoming = WrapIncoming::new(listener.incoming());
        let app = grarr2::app().map_request({
            fn get_peer_addr(
                stream: &Compat<smol::Async<std::net::TcpStream>>,
            ) -> std::net::SocketAddr {
                stream.get_ref().get_ref().peer_addr().unwrap()
            }
            get_peer_addr
        });
        hyper::server::Server::builder(incoming)
            .executor(SmolExecutor)
            .serve(app)
            .await?;
        Ok(())
    })
}
