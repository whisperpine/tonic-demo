mod proto {
    tonic::include_proto!("rpc_demo.nice");
}

use anyhow::Result;
use tonic::Request;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing_subscriber();
    info!("client");

    rpc_say_hello().await?;

    Ok(())
}

fn init_tracing_subscriber() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "client=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn rpc_say_hello() -> Result<()> {
    use proto::greeter_client::GreeterClient;
    use proto::HelloRequest;

    let endpoint = "http://127.0.0.1:6000";
    let mut client = GreeterClient::connect(endpoint).await?;
    debug!("connected to {}", endpoint);

    let request = Request::new(HelloRequest {
        name: "Amiao".to_string(),
    });

    let response = client.say_hello(request).await?;
    let hello_response = response.into_inner();
    debug!(?hello_response);

    Ok(())
}
