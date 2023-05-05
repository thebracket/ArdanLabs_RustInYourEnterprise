use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    spawn,
};

#[derive(Deserialize)]
enum Command {
    SayHello,
}

#[derive(Serialize)]
pub struct Hello {
    message: String,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to bind port");

    loop {
        let (mut socket, _address) = listener
            .accept()
            .await
            .expect("Unable to accept connection");
        spawn(async move {
            loop {
                let size = socket.read_u64().await.expect("Unable to read size");
                let mut buffer = vec![0; size as usize];
                socket.read_exact(&mut buffer).await.expect("Unable to read");
                let command: Command =
                    bincode::deserialize(&buffer).expect("Unable to deserialize");

                if let Command::SayHello = command {
                    let result = Hello {
                        message: "Hello".to_string(),
                    };
                    let result = bincode::serialize(&result).expect("Unable to serialize");
                    socket.write_u64(result.len() as u64).await.expect("Unable to write size");
                    socket.write_all(&result).await.expect("Unable to send");
                }
            }
        });
    }
}
