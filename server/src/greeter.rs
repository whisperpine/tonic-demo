use crate::proto::greeter_server::Greeter;
use crate::proto::{HelloRequest, HelloResponse};
use anyhow::Result;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct GreeterService;

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let hello_request = request.into_inner();
        tracing::debug!(?hello_request);

        let response = Response::new(HelloResponse {
            message: format!("Hello {}", hello_request.name),
        });
        Ok(response)
    }
}
