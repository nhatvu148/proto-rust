use greet::greet_service_client::GreetServiceClient;
use greet::{GreetManyTimesRequest, GreetRequest, Greeting};
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;

pub mod greet {
    tonic::include_proto!("greet");
}

async fn print_response(client: &mut GreetServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let greeting = Greeting {
        first_name: "Kyoko".to_string(),
        last_name: "Murakami".to_string(),
    };

    let request = Request::new(GreetManyTimesRequest {
        greeting: greeting.into(),
    });

    let mut stream = client.greet_many_times(request).await?.into_inner();

    while let Some(response) = stream.message().await? {
        println!("Response = {:?}", response);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreetServiceClient::connect("http://[::1]:50051").await?;

    let greeting = Greeting {
        first_name: "Kyoko".to_string(),
        last_name: "Murakami".to_string(),
    };

    let request = Request::new(GreetRequest {
        greeting: greeting.into(),
    });

    let response = client.greet(request).await?;

    println!("RESPONSE={:?}", response);

    println!("\n*** SERVER STREAMING ***");
    print_response(&mut client).await?;

    Ok(())
}
