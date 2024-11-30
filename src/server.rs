use tonic::{transport::Server, Request, Response, Status};
use greet::greeter_server::{Greeter, GreeterServer};
use greet::{Parm1Request, Parm1Result, Parm2Request, Parm2Result};

pub mod greet {
    tonic::include_proto!("greet");
}

#[derive(Debug, Default)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn add(&self, request: Request<Parm2Request>) -> Result<Response<Parm2Result>, Status> {
        let req = request.into_inner();
        Ok(Response::new(Parm2Result {
            x: req.x,
            y: req.y,
            z: req.x + req.y,
        }))
    }

    async fn mul(&self, request: Request<Parm2Request>) -> Result<Response<Parm2Result>, Status> {
        let req = request.into_inner();
        Ok(Response::new(Parm2Result {
            x: req.x,
            y: req.y,
            z: req.x * req.y,
        }))
    }

    async fn sub(&self, request: Request<Parm2Request>) -> Result<Response<Parm2Result>, Status> {
        let req = request.into_inner();
        Ok(Response::new(Parm2Result {
            x: req.x,
            y: req.y,
            z: req.x - req.y,
        }))
    }

    async fn div(&self, request: Request<Parm2Request>) -> Result<Response<Parm2Result>, Status> {
        let req = request.into_inner();
        if req.y == 0 {
            return Err(Status::invalid_argument("Division by zero"));
        }
        Ok(Response::new(Parm2Result {
            x: req.x,
            y: req.y,
            z: req.x / req.y,
        }))
    }

    async fn pow2(&self, request: Request<Parm1Request>) -> Result<Response<Parm1Result>, Status> {
        let req = request.into_inner();
        Ok(Response::new(Parm1Result {
            x: req.x,
            z: req.x * req.x,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
