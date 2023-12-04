use tonic::{transport::Server, Request, Response, Status};
use tower_http::cors::CorsLayer;
use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};
pub mod hello {
    tonic::include_proto!("hello"); // The string specified here must match the proto package name
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello::HelloReply {
            message: format!("Hello {} ! from gRPC server", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9099".parse()?;
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::permissive()) // Add CORS middleware
        .layer(tonic_web::GrpcWebLayer::new()) // Add gRPC-Web middleware
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}