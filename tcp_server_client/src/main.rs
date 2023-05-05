use std::time::Instant;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Serialize)]
enum Command {
    SayHello,
}

#[derive(Deserialize, Debug)]
pub struct Hello {
    message: String,
}

#[tokio::main]
async fn main() {
    const NUM_REQUESTS: usize = 100_000;
    let mut results = vec![0; NUM_REQUESTS];
    let command = Command::SayHello;
    let command = bincode::serialize(&command).expect("Unable to serialize");
    let mut socket = TcpStream::connect("127.0.0.1:3000")
        .await
        .expect("Unable to connect");

    for n in 0..NUM_REQUESTS {
        let now = Instant::now();
        socket
            .write_u64(command.len() as u64)
            .await
            .expect("Unable to write size");
        socket.write_all(&command).await.expect("Unable to send");
        let receive_size = socket.read_u64().await.expect("Unable to read size");
        let mut buffer = vec![0; receive_size as usize];
        socket.read_exact(&mut buffer).await.expect("Unable to read");
        let message: Hello = bincode::deserialize(&buffer).expect("Unable to deserialize");
        //println!("{message:?}");
        results[n] = now.elapsed().as_micros();
    }

    println!("Average time: {} µs", results.iter().sum::<u128>() / NUM_REQUESTS as u128);
}
