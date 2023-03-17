mod proto {
    tonic::include_proto!("rpc_demo.nice");
}

#[cfg(test)]
mod tests;

pub mod greeter;
pub mod route_guide;
pub mod route_guide_db;

use anyhow::Result;
use proto::greeter_server::GreeterServer;
use tonic::transport::Server;
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
    let greeter = greeter::GreeterService::default();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 6000));
    debug!("listening at {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
