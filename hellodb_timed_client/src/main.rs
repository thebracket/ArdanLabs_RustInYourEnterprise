use serde::Deserialize;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct HelloJson {
    id: i64,
    message: String,
}

#[tokio::main]
async fn main() {
    const NUM_REQUESTS: usize = 10_1000;
    let mut results = vec![0; NUM_REQUESTS];

    let client = reqwest::Client::new();
    for n in 0..NUM_REQUESTS {
        let now = Instant::now();
        let messages: Vec<HelloJson> = client
            .get("http://localhost:3000/")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        results[n] = now.elapsed().as_micros();
    }

    // Ignore the first result, it includes warm-up time
    results.remove(0);

    println!("Average time: {} µs", results.iter().sum::<u128>() / NUM_REQUESTS as u128);
}
