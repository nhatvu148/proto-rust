use greet::greet_service_client::GreetServiceClient;
use greet::{GreetRequest, Greeting};

pub mod greet {
    tonic::include_proto!("greet");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreetServiceClient::connect("http://[::1]:50051").await?;

    let greeting = Greeting {
        first_name: "Kyoko".to_string(),
        last_name: "Murakami".to_string(),
    };

    let request = tonic::Request::new(GreetRequest { greeting: greeting.into() });

    let response = client.greet(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
