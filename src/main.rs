use tonic::{transport::Server, Request, Response, Status};

use crate::api::echo_service_server::{EchoService, EchoServiceServer};
use api::{HelloRequest, Message};

pub mod api {
    tonic::include_proto!("api");
    pub const ECHO_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("echo");
}

#[derive(Debug, Default)]
pub struct EchoAPI {}

#[tonic::async_trait]
impl EchoService for EchoAPI {
    async fn echo(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Message>, Status> {
        let r = request.into_inner();
        let value = r.message.unwrap().value;
        println!("received {:?}", value);
        if value == "test" {
            return Err(Status::invalid_argument("value参数不允许使用test"));
        }

        Ok(Response::new(Message { value: "response".into() }))
    }

    async fn echo1(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = request.into_inner();
        let value = message.value;
        println!("received {:?}", value);
        if value == "test" {
            return Err(Status::invalid_argument("value参数不允许使用test"));
        }

        Ok(Response::new(Message { value: "response".into() }))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8888".parse()?;

    println!("listening on port {:?}", addr);

    // let sss = tonic_reflection::server::Builder::configure()
    let sss = tonic_reflect_protobuf::server::Builder::configure()
        .register_encoded_file_descriptor_set(api::ECHO_DESCRIPTOR_SET)
        .build().unwrap();

    Server::builder()
        .add_service(EchoServiceServer::new(EchoAPI::default()))
        .add_service(sss)
        .serve(addr)
        .await?;

    Ok(())
}