use reqwest::Client;
use serde::Serialize;
use std::time::Instant;
use tokio::{task, time::sleep};
use std::time::Duration;

#[derive(Serialize, Clone)]
struct IncomingRequest {
    account_username: String,
    msisdn: String,
    password: String,
    source: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let total_requests = 100;
    let batch_size = 10;
    let delay_per_batch = Duration::from_secs(60 * 2); // 5 menit
    let endpoint = "http://127.0.0.1:19000/incoming";
    let client = Client::new();

    let start = Instant::now();

    // Buat semua request data
    let all_requests: Vec<IncomingRequest> = (0..total_requests)
        .map(|i| IncomingRequest {
            account_username: "dwi_test".to_string(),
            msisdn: "6289630449253".to_string(),
            password: "12345678".to_string(),
            source: "dwi_test_sender".to_string(),
            message: format!("pesan_test_batching_{}", i),
        })
        .collect();

    // Bagi menjadi batch
    for (batch_index, batch) in all_requests.chunks(batch_size).enumerate() {
        println!("🚀 Memulai batch ke-{} ({} SMS)", batch_index + 1, batch.len());

        let mut handles = Vec::new();
        for (i, req_data) in batch.iter().enumerate() {
            let client = client.clone();
            let endpoint = endpoint.to_string();
            let req_data = req_data.clone();

            let handle = task::spawn(async move {
                match client.post(&endpoint).json(&req_data).send().await {
                    Ok(resp) => {
                        let status = resp.status();
                        println!("[Batch {} - {}] Status: {}", batch_index + 1, i, status);
                    }
                    Err(e) => {
                        println!("[Batch {} - {}] Error: {}", batch_index + 1, i, e);
                    }
                }
            });

            handles.push(handle);
        }

        // Tunggu semua dalam batch selesai
        for h in handles {
            h.await.unwrap();
        }

        // Kalau masih ada batch selanjutnya, delay 5 menit
        if batch_index < (total_requests / batch_size) - 1 {
            println!("⏳ Menunggu 5 menit sebelum batch berikutnya...");
            sleep(delay_per_batch).await;
        }
    }

    println!("✅ Semua selesai dalam {:?}", start.elapsed());
}