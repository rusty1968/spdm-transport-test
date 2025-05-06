use tokio::sync::mpsc as tokio_mpsc;
use async_trait::async_trait;

// Define the MessageBuf and TransportResult types
type MessageBuf<'a> = Vec<u8>;
type TransportResult<T> = Result<T, Box<dyn std::error::Error>>;

// Define the SpdmTransport trait
#[async_trait]
pub trait SpdmTransport {
    async fn send_request<'a>(
        &mut self,
        dest_eid: u8,
        req: &mut MessageBuf<'a>,
    ) -> TransportResult<()>;
    async fn receive_response<'a>(&mut self, rsp: &mut MessageBuf<'a>) -> TransportResult<()>;
    async fn receive_request<'a>(&mut self, req: &mut MessageBuf<'a>) -> TransportResult<()>;
    async fn send_response<'a>(&mut self, resp: &mut MessageBuf<'a>) -> TransportResult<()>;
    fn max_message_size(&self) -> TransportResult<usize>;
    fn header_size(&self) -> usize;
}

// Implement the trait for a struct that holds the sender and receiver
pub struct ChannelTransport {
    sender: tokio_mpsc::Sender<MessageBuf<'static>>,
    receiver: tokio_mpsc::Receiver<MessageBuf<'static>>,
}

impl ChannelTransport {
    pub fn new(
        sender: tokio_mpsc::Sender<MessageBuf<'static>>,
        receiver: tokio_mpsc::Receiver<MessageBuf<'static>>,
    ) -> Self {
        ChannelTransport { sender, receiver }
    }
}

#[async_trait]
impl SpdmTransport for ChannelTransport {
    async fn send_request<'a>(
        &mut self,
        _dest_eid: u8,
        req: &mut MessageBuf<'a>,
    ) -> TransportResult<()> {
        self.sender.send(req.clone()).await?;
        Ok(())
    }

    async fn receive_response<'a>(&mut self, rsp: &mut MessageBuf<'a>) -> TransportResult<()> {
        if let Some(msg) = self.receiver.recv().await {
            *rsp = msg;
            Ok(())
        } else {
            Err("Failed to receive response".into())
        }
    }

    async fn receive_request<'a>(&mut self, req: &mut MessageBuf<'a>) -> TransportResult<()> {
        if let Some(msg) = self.receiver.recv().await {
            *req = msg;
            Ok(())
        } else {
            Err("Failed to receive request".into())
        }
    }

    async fn send_response<'a>(&mut self, resp: &mut MessageBuf<'a>) -> TransportResult<()> {
        self.sender.send(resp.clone()).await?;
        Ok(())
    }

    fn max_message_size(&self) -> TransportResult<usize> {
        Ok(1024) // Example size
    }

    fn header_size(&self) -> usize {
        12 // Example header size
    }
}

