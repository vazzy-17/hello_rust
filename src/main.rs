use tide::{Request, Response, StatusCode, Body};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GreetRequest {
    name: String,
}

#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("Welcome to Rust API!") });

    app.at("/greet").post(|mut req: Request<()>| async move {
        let body: GreetRequest = req.body_json().await?;
        let response = GreetResponse {
            message: format!("Hello, {}!", body.name),
        };

        let mut res = Response::new(StatusCode::Ok);
        res.set_body(Body::from_json(&response)?);
        Ok(res)
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
