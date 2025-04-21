use proto::calculator_client::CalculatorClient;
use std::error::Error;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(url).await?;

    // This is were we set up the values of our variables
    let request = tonic::Request::new(proto::CalculationRequest { a: 4, b: 5 });

    // This is were we choose what function to use
    let response = client.mult(request).await?;

    println!("Response: {:?}", response.get_ref().result);

    Ok(())
}
