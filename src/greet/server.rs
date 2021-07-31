use tonic::{transport::Server, Request, Response, Status};

use greet::greet_service_server::{GreetService, GreetServiceServer};
use greet::{GreetRequest, GreetResponse, Greeting};

pub mod greet {
    tonic::include_proto!("greet");
}

#[derive(Default)]
pub struct MyGreetService {}

#[tonic::async_trait]
impl GreetService for MyGreetService {
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let greeting = Greeting {
            first_name: match &request.get_ref().greeting {
                Some(gr) => gr.first_name.clone(),
                None => "".to_string(),
            },
            last_name: match &request.get_ref().greeting {
                Some(gr) => gr.last_name.clone(),
                None => "".to_string(),
            },
        };

        let reply = GreetResponse {
            result: format!("Hello {} {}!", greeting.first_name, greeting.last_name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greet = MyGreetService::default();

    println!("GreetServiceServer listening on {}", addr);

    Server::builder()
        .add_service(GreetServiceServer::new(greet))
        .serve(addr)
        .await?;

    Ok(())
}
