use tide::{Request, Response, StatusCode, Body};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, FromRow};
use std::env;

#[derive(Deserialize)]
struct UserRequest {
    id: i32, // Hanya menerima ID dari request body
}

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    username: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL harus diset di build.rs");
    let listen_address = env::var("LISTEN_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let mut app = tide::with_state(pool);

    app.at("/user").post(get_user_by_id);

    app.listen(listen_address).await?;
    Ok(())
}

async fn get_user_by_id(mut req: Request<PgPool>) -> tide::Result {
    // 🔹 Ambil JSON dulu, sebelum meminjam `req.state()`
    let body_json = req.body_json::<UserRequest>().await;

    let body = match body_json {
        Ok(b) => b,
        Err(_) => {
            let mut res = Response::new(StatusCode::BadRequest);
            res.set_body(Body::from_string("Invalid JSON. Please provide a valid user ID.".to_string()));
            return Ok(res);
        }
    };

    // 🔹 Sekarang ambil database pool setelah `req` tidak digunakan lagi
    let pool = req.state();

    // 🔹 Query ke database untuk mendapatkan user berdasarkan ID
    match sqlx::query_as::<_, User>("SELECT id, name, username FROM users WHERE id = $1")
        .bind(body.id)
        .fetch_one(pool)
        .await
    {
        Ok(user) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&user)?);
            Ok(res)
        },
        Err(sqlx::Error::RowNotFound) => {
            let mut res = Response::new(StatusCode::NotFound);
            res.set_body(Body::from_string(format!("User with ID {} not found", body.id)));
            Ok(res)
        },
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            let mut res = Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_string("Internal Server Error".to_string()));
            Ok(res)
        }
    }
}
