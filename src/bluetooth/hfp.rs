use bluer::rfcomm::ProfileHandle;
use futures::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct HFP {
    pub hfp_handle: ProfileHandle,
}

impl HFP {
    pub async fn handle_hfp_profile(mut self) -> anyhow::Result<()> {
        while let Some(req) = self.hfp_handle.next().await {
            if let Ok(mut stream) = req.accept() {
                println!("Socket opened! Ready for data.");

                stream.write(b"AT+BRSF=0\r").await?;

                let mut buffer = vec![0; 1024];

                loop {
                    match stream.read(&mut buffer).await {
                        Ok(0) => {
                            println!("The device disconnected.");
                            break;
                        }
                        Ok(n) => {
                            // We received 'n' bytes of data!
                            let data = String::from_utf8_lossy(&buffer[..n]);

                            println!("Received {} bytes: {}", n, data);
                        }
                        Err(e) => {
                            println!("Error reading from stream: {}", e);
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
