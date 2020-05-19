#[derive(Debug, Clone)]
struct SmolSpawner;

impl futures::task::Spawn for SmolSpawner {
    fn spawn_obj(
        &self, 
        future: futures::future::FutureObj<'static, ()>
    ) -> Result<(), futures::task::SpawnError> {
        smol::Task::spawn(future).detach();
        Ok(())
    }

    fn status(&self) -> Result<(), futures::task::SpawnError> {
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    smol::run(async {
        http_service_hyper::Server::builder(smol::Async::<std::net::TcpListener>::bind("127.0.0.1:8080")?.incoming()).with_spawner(SmolSpawner).serve(grarr2::app()).await?;
        Ok(())
    })
}
