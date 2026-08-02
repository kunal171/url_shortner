use axum::{routing::{get, post},
    Router,
};
use url_shortner::api::health;
use url_shortner::database::connect_db;

#[tokio::main]
async fn main() {
    //New App router
    let app = Router::new().route("/health", get(health));

    //Bind the TCP listener to the address and port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind server");

    println!("Server Running on http://127.0.0.1:3000");

    //Seting up DB connection
    let database = match connect_db().await {
        Ok(database) => database,
        Err(err) => {
            eprintln!("failed to connect to the Database: {err}");
            return;
        }
    }
    println!("connected to database: {}", database.name());


    axum::serve(listener, app)
        .await
        .expect("server Failed")
}