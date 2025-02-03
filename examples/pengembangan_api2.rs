use tide::{Request, Response, StatusCode, Body};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Deserialize)]
struct GreetRequest {
    name: Option<String>,
}

#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

#[derive(Serialize, sqlx::FromRow)]
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

    app.at("/").get(|_| async { Ok("Welcome to Rust API!") });

    app.at("/greet").get(|req: Request<sqlx::PgPool>| async move {
        let name = req.param("name").unwrap_or("Guest").to_string();
        let response = GreetResponse {
            message: format!("Hello, {}!", name),
        };

        let mut res = Response::new(StatusCode::Ok);
        res.set_body(Body::from_json(&response)?);
        Ok(res)
    });

    app.at("/greet").post(|mut req: Request<sqlx::PgPool>| async move {
        let body: GreetRequest = req.body_json().await.unwrap_or(GreetRequest { name: None });
        let name = body.name.unwrap_or("Guest".to_string());
        let response = GreetResponse {
            message: format!("Hello, {}!", name),
        };

        let mut res = Response::new(StatusCode::Ok);
        res.set_body(Body::from_json(&response)?);
        Ok(res)
    });

    app.at("/user/:id").get(|req: Request<sqlx::PgPool>| async move {
        let id: i32 = req.param("id").unwrap_or("0").parse().unwrap_or(0);
        let pool = req.state();

        match sqlx::query_as::<_, User>("SELECT id, name, username FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
        {
            Ok(user) => {
                let mut res = Response::new(StatusCode::Ok);
                res.set_body(Body::from_json(&user)?);
                Ok(res)
            },
            Err(_) => {
                let mut res = Response::new(StatusCode::NotFound);
                res.set_body(Body::from_string("User not found".to_string()));
                Ok(res)
            },
        }
    });

    app.listen(listen_address).await?;
    Ok(())
}
