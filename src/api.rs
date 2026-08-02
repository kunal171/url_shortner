use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    Json,
};

use mongodb::{
    bson::{doc, DateTime},
    Database,
};

use crate::url::{
    generate_short_code, CreateShortUrlRequest, CreateShortUrlResponse, UrlStatsResponse, 
    UrlRecord, validate_long_url,
};
use crate::error::{ErrorResponse, error_response};


#[derive(Clone)] 
pub struct AppState {
    // MongoDB database handle shared by handlers.
    pub database: Database,

    // Base URL like http://127.0.0.1:3000.
    pub base_url: String,
}

pub async fn health() -> &'static str {
    //  simplest endpoint to check that the server is alive
    "ok"
}

pub async fn create_short_url(
    State(state): State<AppState>,
    Json(payload): Json<CreateShortUrlRequest>,
) -> Result<Json<CreateShortUrlResponse>, (StatusCode, Json<ErrorResponse>)> {

    // Reject invalid URLs before generating or saving anything.
    validate_long_url(&payload.long_url)
        .map_err(|message| {
            error_response(
                StatusCode::BAD_REQUEST,
                "invalid_url",
                &message,
            )
        })?;

    //Generate the Random short code for the URL.
    let code = generate_short_code();

    // Build the MongoDB document we want to save.
    let record = UrlRecord {
        id: None,
        code: code.clone(),
        long_url: payload.long_url.clone(),
        visits: 0,
        created_at: DateTime::now(),
    };

    // Select the MongoDB collection.
    let collection = state.database.collection::<UrlRecord>("urls");

    collection
        .insert_one(record)
        .await
        .map_err(|_| {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                "failed to save short URL",
            )
        })?;

    // Build the full short URL returned to the client.
    let short_url = format!("{}/{}", state.base_url, code);

    // Return JSON response.
    Ok(Json(CreateShortUrlResponse {
        code,
        short_url,
        long_url: payload.long_url,
    }))
}

pub async fn redirect_short_url(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Select the MongoDB collection.
    let collection = state.database.collection::<UrlRecord>("urls");

    // Find URL record by short code.
    let record = collection
        .find_one(doc! { "code": &code })
        .await
        .map_err(|_| {
            error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database_error",
            "object not found",
            )
        })?;

     // If code does not exist, return 404.
    let Some(record) = record else {
        return Err(error_response(
            StatusCode::NOT_FOUND,
            "not_found",
            "short URL code does not exist",
        ));
    };

    // Increase visit count by 1.
    collection
        .update_one(
            doc! { "code": &code },
            doc! { "$inc": { "visits": 1 } },
        )
        .await
        .map_err(|_| {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                "failed to increase the count",
            )
        })?;


    let mut headers = HeaderMap::new();

    // Set Location header to original long URL.
    let location = HeaderValue::from_str(&record.long_url)
        .map_err(|_| {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Redirect error ",
                "failed to set redirect url",
            )
        })?;

    headers.insert(header::LOCATION, location);

    // Return 302 redirect.
    Ok((StatusCode::FOUND, headers))
}


pub async fn get_url_stats(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<UrlStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Select the MongoDB collection.
    let collection = state.database.collection::<UrlRecord>("urls");

    // Find URL record by short code.
    let record = collection
        .find_one(doc! { "code": &code })
        .await
        .map_err(|_| {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                "Object Not found",
            )
        })?;

     // If code does not exist, return 404.
    let Some(record) = record else {
        return Err(error_response(
            StatusCode::NOT_FOUND,
            "not_found",
            "short URL code does not exist",
        ));
    };

    // Build full short URL.
    let short_url = format!("{}/{}", state.base_url, record.code);

    // Return stats as JSON.
    Ok(Json(UrlStatsResponse {
        code: record.code,
        long_url: record.long_url,
        short_url,
        visits: record.visits,
        created_at: record.created_at,
    }))
}