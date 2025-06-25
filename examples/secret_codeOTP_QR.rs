use tide::{Request, Response, StatusCode};
use qrcode::QrCode;
use qrcode::types::Color;
use image::{Luma, ImageBuffer};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;
use rand::RngCore;
use data_encoding::BASE32;
use std::io::Cursor;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/qr").get(show_qr_code);

    println!("Server running at http://127.0.0.1:1945");
    app.listen("127.0.0.1:1945").await?;
    Ok(())
}

async fn show_qr_code(_req: Request<()>) -> tide::Result {
    let email = "user@example.com";
    let issuer = "MyRustApp";

    // Generate random 20-byte secret
    let mut secret_bytes = [0u8; 20];
    rand::thread_rng().fill_bytes(&mut secret_bytes);
    let secret_base32 = BASE32.encode(&secret_bytes);

    // Build otpauth URL
    let otp_url = format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA1&digits=6&period=30",
        issuer, email, secret_base32, issuer
    );

    // Generate QR code bitmap
    let code = QrCode::new(otp_url.as_bytes()).unwrap();
    let width = code.width();
    let scale = 10; // pixels per module
    let imgx = width * scale;
    let imgy = width * scale;

    let mut img = ImageBuffer::<Luma<u8>, Vec<u8>>::new(imgx as u32, imgy as u32);
    for x in 0..width {
        for y in 0..width {
            let color = if code[(x, y)] == Color::Dark { 0 } else { 255 };
            for dx in 0..scale {
                for dy in 0..scale {
                    img.put_pixel(
                        (x * scale + dx) as u32,
                        (y * scale + dy) as u32,
                        Luma([color]),
                    );
                }
            }
        }
    }

    // Encode to PNG
let mut png_bytes = Cursor::new(Vec::new());
image::DynamicImage::ImageLuma8(img)
    .write_to(&mut png_bytes, image::ImageOutputFormat::Png)
    .unwrap();

let png_bytes = png_bytes.into_inner(); // ambil hasilnya

    // Encode to base64
    let base64_img = BASE64.encode(&png_bytes);

    let html = format!(
        r#"
        <h2>Scan This QR with Google Authenticator</h2>
        <img src="data:image/png;base64,{}" alt="QR Code" />
        <p><strong>Manual Entry Secret:</strong> {}</p>
        "#,
        base64_img, secret_base32
    );

    Ok(Response::builder(StatusCode::Ok)
        .body(html)
        .content_type("text/html")
        .build())
}
