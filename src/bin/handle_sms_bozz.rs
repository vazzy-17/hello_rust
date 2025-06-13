use tide::Request as TideRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Local;

#[derive(Debug, Deserialize)]
struct SmsGwRequestInner {
    account: String,
    password: String,
    mobile: String,
    content: String,
    sender: String,
}


async fn send_delivery_report(mobile: &str, drstatus: &str, drresult: &str,msgid: &str){
    let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

     // 🔹 Gunakan `encode()` untuk mencegah masalah encoding di URL
    let url = format!(
        "http://127.0.0.1:19100/bozz?mobile={}&drstatus={}&drresult={}&drtime={}&msgid={}",
        mobile,
        drstatus,
        drresult,
        &date,
        msgid
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


async fn handle_sms_request(mut req: TideRequest<()>) -> tide::Result {
    let body: SmsGwRequestInner = req.body_json().await?;
    log::info!("Received SMS Request: {:?}", body);

        let msgid = Uuid::new_v4().to_string();
    log::info!("Generated msgid: {}", msgid);



     
    send_delivery_report(&body.mobile, "2", "Delivered", &msgid).await;

    // Simulasi respons (bisa sesuaikan status sesuai logika parsing di `parse_resp`)
    let response = serde_json::json!({
     
            "msgid" : msgid,
            "status": "100", // success
            "info" : "success"
        
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