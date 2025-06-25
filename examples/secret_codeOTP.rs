// use async_std::task;
use tide::{Request, Response, StatusCode};
use tide::{http::headers::CONTENT_TYPE, http::Mime};
// use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, TOTP};
use std::collections::HashMap;
use base32::{Alphabet, decode};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/").get(show_login_form);
    app.at("/login").post(handle_login);

    println!("Server running at http://127.0.0.1:1945");
    app.listen("127.0.0.1:1945").await?;
    Ok(())
}

const SECRET: &str = "JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP"; // base32 encoded secret

async fn show_login_form(_req: Request<()>) -> tide::Result {
    let html = r#"
        <h2>Login with Google Authenticator</h2>
        <form action="/login" method="post">
            <label>Auth Code:</label><br>
            <input type="text" name="code" maxlength="6" required>
            <br><br>
            <button type="submit">Login</button>
        </form>
    "#;

    Ok(Response::builder(StatusCode::Ok)
        .body(html)
        .content_type("text/html")
        .build())
}

async fn handle_login(mut req: Request<()>) -> tide::Result {
    let form_data = req.body_form::<HashMap<String, String>>().await?;
    let code_input = form_data.get("code").cloned().unwrap_or_default();
    let code = code_input.trim();
    let secret_bytes = decode(Alphabet::RFC4648 { padding: false }, SECRET)
    .expect("Failed to decode base32 secret");

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,       // digits
        1,       // skew
        30,      // period in seconds
           secret_bytes,// base32-encoded secret
    ).unwrap();

    if totp.check_current(code).unwrap_or(false) {
        respond(StatusCode::Ok, "✅ Login success!")
    } else {
        respond(StatusCode::Unauthorized, "❌ Invalid code.")
    }
}

fn respond(status: StatusCode, message: &str) -> tide::Result<Response> {
    let mut res = Response::new(status);
    res.set_body(message);
    res.insert_header(CONTENT_TYPE, Mime::from("text/plain"));
    Ok(res)
}
