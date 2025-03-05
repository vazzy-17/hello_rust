use tide::security::{CorsMiddleware, Origin};
use tide::{Request, Result, StatusCode, Server};
use async_std::task;
use serde::Deserialize;
use tide::http::headers::HeaderValue; // Import HeaderValue
use std::str::FromStr; // Import FromStr trait

#[derive(Debug, Deserialize)]
struct SmscQuery {
    smsc: String,
    password: String,
}

#[async_std::main]
async fn main() -> Result<()> {
    // Create two server instances for two ports
    let mut app1 = create_app();
    let mut app2 = create_app();

    println!("Servers running at:");
    println!("👉 http://127.0.0.1:33001");
    println!("👉 http://127.0.0.1:33002");

    // Run both servers concurrently
    let server1 = task::spawn(async { app1.listen("127.0.0.1:33000").await });
    let server2 = task::spawn(async { app2.listen("127.0.0.1:33100").await });

    // Wait for both servers to run
    server1.await?;
    server2.await?;

    Ok(())
}

// Function to create a server with CORS middleware
fn create_app() -> Server<()> {
    let mut app = tide::new();

    let cors = CorsMiddleware::new()
        .allow_origin(Origin::from("*")) // Allow all domains
        .allow_methods(HeaderValue::from_str("GET,POST").unwrap()) // Allow GET and POST methods as a string
        .allow_headers(HeaderValue::from_str("Content-Type").unwrap()) // Allow specific headers as a string
        .allow_credentials(false); // Not using credentials

    app.with(cors);
    app.at("/add-smsc").get(handle_get);


    app
}

// Handler GET to handle requests from frontend
async fn handle_get(req: Request<()>) -> Result<String> {
    let query: SmscQuery = req.query().map_err(|_| {
        tide::Error::from_str(StatusCode::BadRequest, "Invalid query parameters")
    })?;

    Ok(format!(
        "Received GET request:\nSMSC: {}\nPassword: {}",
        query.smsc, query.password
    ))
}

