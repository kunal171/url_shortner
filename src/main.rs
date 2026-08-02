use axum::{routing::{get, post},
    Router,
};
use url_shortner::api::{create_short_url,redirect_short_url,get_url_stats, health, AppState};
use url_shortner::database::connect_db;

#[tokio::main]
async fn main() {

    //URL of App
    let base_url = std::env::var("BASE_URL")
    .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());

    //Seting up DB connection
    let database = match connect_db().await {
        Ok(database) => database,
        Err(err) => {
            eprintln!("failed to connect to the Database: {err}");
            return;
        }
    };


    println!("connected to database: {}", database.name());

    let state = AppState {
        database,
        base_url: base_url.clone(),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/shorten", post(create_short_url))
        .route("/api/urls/{code}/stats", get(get_url_stats))
        .route("/{code}", get(redirect_short_url))
        .with_state(state);

    
    let bind_address = "127.0.0.1:3001";
    //Bind the TCP listener to the address and port
    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .expect("failed to bind server");

    println!("Server Running on http://{}", &bind_address);


    axum::serve(listener, app)
        .await
        .expect("server Failed")
}