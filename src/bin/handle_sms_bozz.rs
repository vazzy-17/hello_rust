use tide::Request as TideRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct SmsGwRequestInner {
    accoount: String,
    password: String,
    mobile: String,
    content: String,
    sender: String,
}

#[derive(Debug, Deserialize)]
struct SmsGwRequest {
    smsgwRq: SmsGwRequestInner,
}

async fn handle_sms_request(mut req: TideRequest<()>) -> tide::Result {
    let body: SmsGwRequest = req.body_json().await?;
    log::info!("Received SMS Request: {:?}", body);

    // Simulasi respons (bisa sesuaikan status sesuai logika parsing di `parse_resp`)
    let response = serde_json::json!({
        "smsgwRs": {
            "msgid" : "inimsgid",
            "status": "100", // success
            "info" : "success"
        }
    });

    Ok(tide::Response::builder(200)
        .content_type(tide::http::mime::JSON)
        .body(response.to_string())
        .build())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Jalankan HTTP API POST receiver
    // async_std::task::spawn(async {
        let mut app = tide::new();
        app.at("/api/sms").post(handle_sms_request);
        app.listen("127.0.0.1:8080").await.unwrap();
    // });


    Ok(())
}