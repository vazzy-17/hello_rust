use tide::Request;
use serde::{Deserialize};

#[derive(Debug,Deserialize)]

struct param{
    msisdn: String,
    message: String,
}

async fn terima (mut req:Request<()>) -> tide::Result{
    let body: param = req.body_json().await?;
     println!("Received via println!: {:?}", body);
    log::info!("Received Message : {:?}",body);

    let response=serde_json::json!({
        "msisdn" : body.msisdn,
        "message" : body.message,
        "status" : "Mi messenger aktif"
    });

    Ok(tide::Response::builder(200)
    .content_type(tide::http::mime::JSON)
    .body(response.to_string())
    .build())
}

#[async_std::main]
async fn main() -> tide::Result<()>
{
    let mut app=tide::new();
    app.at("/terima").post(terima);
    app.listen("127.0.0.1:8080").await.unwrap();

    Ok(())
}