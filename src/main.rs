use tide::{Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use surf;
use chrono::Local;
use urlencoding::encode; // 🔹 Pastikan URL aman dengan encoding

#[derive(Debug, Deserialize)]
struct SmsGwRequest {
    smsgwRq: SmsGwData,
}

#[derive(Debug, Deserialize)]
struct SmsGwData {
    username: String,
    password: String,
    msisdn: String,
    message: String,
    sender: String,
    chanel: String,
}

#[derive(Debug, Serialize)]
struct SmsResponse {
    ErrorCode: i32,
    Description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sms: Option<SmsDetail>,
}

#[derive(Debug, Serialize)]
struct SmsDetail {
    Id: i32,
    Sender: String,
    Destination: String,
    MessageCount: i32,
}

async fn send_delivery_report(message_id: &str, msisdn: &str, sender: &str, dlrstatus: &str, description: &str) {
    let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 🔹 Gunakan `encode()` untuk mencegah masalah encoding di URL
    let url = format!(
        "http://0.0.0.0:19100/sharidaya?messageid={}&msisdn={}&sender={}&dlrstatus={}&description={}&date={}",
        encode(message_id),
        encode(msisdn),
        encode(sender),
        encode(dlrstatus),
        encode(description),
        encode(&date)
    );

    println!("📡 Sending request to: {}", url); // 🔹 Debugging log sebelum request

    match surf::get(&url).await {
        Ok(mut response) => {
            let body = response.body_string().await.unwrap_or_else(|_| "No response body".to_string());
            println!("✅ Delivery report sent successfully. Response: {}", body);
        }
        Err(err) => {
            eprintln!("❌ Gagal mengirim delivery report: {:?}", err);
        }
    }
}

async fn handle_sms(mut req: Request<()>) -> tide::Result {
    let body: SmsGwRequest = match req.body_json().await {
        Ok(data) => data,
        Err(_) => {
            let response = SmsResponse {
                ErrorCode: 3,
                Description: "Pesan tidak valid (kosong)".to_string(),
                sms: None,
            };
            return Ok(Response::builder(StatusCode::BadRequest)
                .body(serde_json::to_string(&response)?)
                .content_type("application/json")
                .build());
        }
    };
    
    println!("📩 Received SMS Request: {:#?}", body);
    
    let message_id = "241050";
    let dlrstatus = "Delivered";
    let description = "Pesan berhasil dikirim";
    
    send_delivery_report(message_id, &body.smsgwRq.msisdn, &body.smsgwRq.sender, dlrstatus, description).await;

    let response = SmsResponse {
        ErrorCode: 0,
        Description: "OK".to_string(),
        sms: Some(SmsDetail {
            Id: 241050,
            Sender: body.smsgwRq.sender.clone(),
            Destination: body.smsgwRq.msisdn.clone(),
            MessageCount: 1,
        }),
    };
    
    Ok(Response::builder(StatusCode::Ok)
        .body(serde_json::to_string(&response)?)
        .content_type("application/json")
        .build())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/xyz").post(handle_sms);
    
    println!("🚀 Server running on 127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
