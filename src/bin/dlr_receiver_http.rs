use tide::{Request, Response, StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LoginQuery {
    trx_id: String,
    status: String,
    ref_id: String,
    sms_count: String,
    sms_parts: String,
    sms_length: String,
}

async fn login(req: Request<()>) -> tide::Result {
    let query: LoginQuery = req.query()?;  // ✅ Ambil dari query string
    println!(
        "trx_id: {}, status: {}
         , ref_id: {}, sms_count: {}, sms_parts: {},sms_length: {}
        ",
        query.trx_id, query.status
        , query.ref_id, query.sms_count, query.sms_parts, query.sms_length
    );
// , ref_id: {}, sms_count: {}, sms_parts: {}
    Ok(Response::new(StatusCode::Ok))
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/login").get(login);  // GET OK ✅
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}