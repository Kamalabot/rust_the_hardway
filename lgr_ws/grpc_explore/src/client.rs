// run client with cargo run --bin payments-client

use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(
        BtcPaymentRequest {
            from_addr: "123456".to_owned(),
            to_addr: "567856".to_owned(),
            amount:42
        }
    );
    let response = client.send_payment(request).await?;
    println!("Response= {:?}", response);
    Ok(())
}
