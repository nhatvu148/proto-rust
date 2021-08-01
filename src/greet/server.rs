use greet::greet_service_server::{GreetService, GreetServiceServer};
use greet::{GreetManyTimesRequest, GreetManytimesResponse, GreetRequest, GreetResponse, Greeting};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

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

    type GreetManyTimesStream = ReceiverStream<Result<GreetManytimesResponse, Status>>;

    async fn greet_many_times(
        &self,
        request: Request<GreetManyTimesRequest>,
    ) -> Result<Response<Self::GreetManyTimesStream>, Status> {
        println!("GreetManyTimes function was invoked with {:?}", request);
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

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for i in 0..10 {
                let response = GreetManytimesResponse {
                    result: format!(
                        "Hello {} {} number {}!",
                        greeting.first_name, greeting.last_name, i
                    ),
                };
                tx.send(Ok(response.clone())).await.unwrap();
                sleep(Duration::from_millis(1000)).await;
            }

            println!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
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
