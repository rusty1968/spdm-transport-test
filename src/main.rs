use spdm_transport_test::{SpdmTransport, ChannelTransport};
use tokio::sync::mpsc as tokio_mpsc;
#[tokio::main]
async fn main() {
    // Create a channel
    let (tx, rx) = tokio_mpsc::channel(100);

    // Create a ChannelTransport instance
    let mut transport = ChannelTransport::new(tx, rx);

    // Example usage
    let mut request = vec![1, 2, 3];
    transport.send_request(1, &mut request).await.unwrap();

    let mut response = vec![];
    transport.receive_response(&mut response).await.unwrap();
    println!("Received response: {:?}", response);
}
