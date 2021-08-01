use futures::stream;
use greet::greet_service_client::GreetServiceClient;
use greet::{GreetManyTimesRequest, GreetRequest, Greeting, LongGreetRequest};
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;

pub mod greet {
    tonic::include_proto!("greet");
}

async fn server_stream(client: &mut GreetServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
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

async fn client_stream(client: &mut GreetServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let greetings = vec![
        LongGreetRequest {
            greeting: (Greeting {
                first_name: "Kyoko".to_string(),
                last_name: "Murakami".to_string(),
            })
            .into(), // Convert into Option?
        },
        LongGreetRequest {
            greeting: (Greeting {
                first_name: "Murakami".to_string(),
                last_name: "Kyoko".to_string(),
            })
            .into(),
        },
        LongGreetRequest {
            greeting: (Greeting {
                first_name: "Nhat".to_string(),
                last_name: "Vu".to_string(),
            })
            .into(),
        },
        LongGreetRequest {
            greeting: (Greeting {
                first_name: "Akiyama".to_string(),
                last_name: "Murakami".to_string(),
            })
            .into(),
        },
        LongGreetRequest {
            greeting: (Greeting {
                first_name: "Kyoko".to_string(),
                last_name: "Akiyama".to_string(),
            })
            .into(),
        },
    ];

    let request = Request::new(stream::iter(greetings));

    match client.long_greet(request).await {
        Ok(response) => println!("SUMMARY: {:?}", response.into_inner()),
        Err(e) => println!("error while calling LongGreet: {:?}", e),
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
    server_stream(&mut client).await?;

    println!("\n*** CLIENT STREAMING ***");
    client_stream(&mut client).await?;

    Ok(())
}
