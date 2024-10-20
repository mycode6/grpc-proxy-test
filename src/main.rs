use tonic::{transport::Server, Request, Response, Status};

use crate::api::echo_service_server::{EchoService, EchoServiceServer};
use api::Message;

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug, Default)]
pub struct EchoAPI {}

#[tonic::async_trait]
impl EchoService for EchoAPI {
    async fn echo(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = request.into_inner();

        println!("received {:?}", message.value);

        Ok(Response::new(Message { value: "response".into() }))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8888".parse()?;

    println!("listening on port {:?}", addr);

    let sss = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!("/Users/codeman/Documents/Github/mycode6/grpc-proxy-test/bin"))
        .build_v1alpha().unwrap();

    Server::builder()
        .add_service(EchoServiceServer::new(EchoAPI::default()))
        .add_service(sss)
        .serve(addr)
        .await?;

    Ok(())
}