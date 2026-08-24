use mongodb::bson::{DateTime, oid::ObjectId};
use rand::{RngExt, distr::Alphanumeric};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlRecord {
    // MongoDB document id.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    //Short Code like "anvCdxQ"
    pub code: String,

    //Original Url
    pub long_url: String,

    // Number of times short URL was opened
    pub visits: i64,

    // Creation timestamp.
    pub created_at: DateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateShortUrlRequest {
    // URL sent by client.
    pub long_url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateShortUrlResponse {
    // Generated short code.
    pub code: String,

    // Full short URL.
    pub short_url: String,

    // Original URL.
    pub long_url: String,
}

#[derive(Debug, Serialize)]
pub struct UrlStatsResponse {
    // Short code for this URL.
    pub code: String,

    // Original long URL.
    pub long_url: String,

    // Full short URL.
    pub short_url: String,

    // Number of redirects so far.
    pub visits: i64,

    // Creation timestamp from MongoDB.
    pub created_at: DateTime,
}

pub fn generate_short_code() -> String {
    //Generate the 7-character random code like "aB9xK2p"
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

pub fn validate_long_url(long_url: &str) -> Result<(), String> {
    // Try to parse the input as a URL.
    let parsed_url = url::Url::parse(long_url).map_err(|_| "invalid URL format".to_string())?;

    // Only allow http and https links.
    match parsed_url.scheme() {
        "http" | "https" => Ok(()),
        _ => Err("only http and https URLs are allowed".to_string()),
    }
}
