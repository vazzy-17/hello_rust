use reqwest::Client;
use serde::Serialize;
use std::time::Instant;
use tokio::task;

#[derive(Serialize)]
struct IncomingRequest {
    account_username: String,
    msisdn: String,
    password: String,
    source: String,
    message: String,
}

// test hit api secara bersamaan

#[tokio::main]
async fn main() {
    let total_requests = 100;
    let concurrent = 100;
    let endpoint = "http://127.0.0.1:19000/incoming";

    let client = Client::new();
    let start = Instant::now();

    let mut handles = Vec::new();

    for i in 0..total_requests {
        let client = client.clone();
        let endpoint = endpoint.to_string();

        let req_data = IncomingRequest {
            account_username: "dwi_test".to_string(),
            msisdn: "6289630449253".to_string(),
            password: "2GVcTANh".to_string(),
            source: "dwi_test_sender".to_string(),
            message: format!("pesan_test_berantai_sebanyak_100_bersamaan_{}", i),
        };

        // spawn async task
        let handle = task::spawn(async move {
            match client
                .post(&endpoint)
                .json(&req_data)
                .send()
                .await
            {
                Ok(resp) => {
                    let status = resp.status();
                    println!("[{}] Status: {}", i, status);
                    status.as_u16()
                }
                Err(e) => {
                    println!("[{}] Error: {}", i, e);
                    0
                }
            }
        });

        handles.push(handle);

        // batasi jumlah concurrent
        if handles.len() >= concurrent {
            for h in handles.drain(..) {
                h.await.unwrap();
            }
        }
    }

    // tunggu sisanya
    for h in handles {
        h.await.unwrap();
    }

    let duration = start.elapsed();
    println!("Selesai dalam: {:?}", duration);
}