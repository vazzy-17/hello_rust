use tide::{Request, Response, StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SmscParams {
    smsc: String,
    password: String,
}

async fn handle_add_smsc(req: Request<()>) -> tide::Result<Response> {
    // Ambil query parameter dari URL
    let params: SmscParams = req.query()?;

    // Log parameter
    log::info!("Received smsc: {}, password: {}", params.smsc, params.password);

    // Lakukan logika sesuai kebutuhan, misalnya simpan ke DB atau validasi

    // Buat response JSON
    let json = serde_json::json!({
        "status": "success",
        "message": format!("SMS Center {} berhasil diproses", params.smsc)
    });

    Ok(Response::builder(StatusCode::Ok)
        .content_type(tide::http::mime::JSON)
        .body(serde_json::to_string(&json)?)
        .build())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start(); // Untuk logging
    let mut app = tide::new();

    app.at("/add-smsc").get(handle_add_smsc);

    println!("Server berjalan di http://127.0.0.1:33100");
    app.listen("127.0.0.1:33100").await?;
    Ok(())
}
