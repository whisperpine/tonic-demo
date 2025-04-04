// rustc
#![cfg_attr(debug_assertions, allow(unused))]
// #![cfg_attr(not(debug_assertions), deny(missing_docs))]
// #![cfg_attr(not(debug_assertions), deny(clippy::unwrap_used))]
#![cfg_attr(not(debug_assertions), deny(warnings))]
// clippy
#![cfg_attr(not(debug_assertions), deny(clippy::todo))]
#![cfg_attr(
    not(any(test, debug_assertions)),
    deny(clippy::print_stdout, clippy::dbg_macro)
)]

pub mod proto {
    tonic::include_proto!("rpc_demo.nice");
}

#[cfg(test)]
mod tests;

pub mod greeter;
pub mod route_guide;
pub mod route_guide_db;

use anyhow::Result;
use proto::greeter_server::GreeterServer;
use proto::route_guide_server::RouteGuideServer;
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
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn serve() -> Result<()> {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 6000));
    debug!("listening at {}", addr);

    let greeter = greeter::GreeterService;
    let route_guide = route_guide::RouteGuideService::new(route_guide_db::load()?);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(RouteGuideServer::new(route_guide))
        .serve(addr)
        .await?;

    Ok(())
}
