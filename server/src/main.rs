mod proto {
    tonic::include_proto!("rpc_demo.nice");
}

use anyhow::Result;
use proto::greeter_server::{Greeter, GreeterServer};
use proto::{HelloRequest, HelloResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing_subscriber();
    info!("server");

    serve().await?;

    Ok(())
}

fn init_tracing_subscriber() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn serve() -> Result<()> {
    let greeter = MyGreeter::default();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 6000));
    debug!("listening at {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Default)]
struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let hello_request = request.into_inner();
        debug!(?hello_request);

        let response = Response::new(HelloResponse {
            message: format!("Hello {}", hello_request.name),
        });
        Ok(response)
    }
}
