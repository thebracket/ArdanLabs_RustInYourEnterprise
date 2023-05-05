use serde::Serialize;
use std::{time::Instant, hint::black_box};

#[derive(Serialize, Debug)]
struct HelloJson {
    id: i64,
    message: String,
}

fn main() {
    const NUM_REQUESTS: usize = 10_1000;
    let mut results = vec![0; NUM_REQUESTS];

    let messages = vec![HelloJson {
        id: 1,
        message: "Hello, world!".to_string(),
    },
    HelloJson {
        id: 2,
        message: "Hello, galaxy!".to_string(),
    },
    HelloJson {
        id: 3,
        message: "Hello, universe!".to_string(),
    }];

    for n in 0..NUM_REQUESTS {
        let now = Instant::now();
        black_box(serde_json::to_string(&messages).unwrap());
        results[n] = now.elapsed().as_nanos();
    }

    // Ignore the first result, it includes warm-up time
    results.remove(0);

    println!("Average time: {} nanoseconds", results.iter().sum::<u128>() / NUM_REQUESTS as u128);
}
