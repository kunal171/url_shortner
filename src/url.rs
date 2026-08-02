use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[drive(Debug,Serialize, Deserialize)]
struct UrlRecord {
    // MongoDB document id.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    //Short Code like "anvCdxQ"
    code: String,

    //Original Url
    long_url: String,

    // Number of times short URL was opened 
    visits: i64,

    // Creation timestamp.
    created_at: DateTime,
}

#[derive(Debug, Deserialize)]
struct CreateShortUrlRequest {
    // URL sent by client.
    long_url: String,
}

#[derive(Debug, Serialize)]
struct CreateShortUrlResponse {
    // Generated short code.
    code: String,

    // Full short URL.
    short_url: String,

    // Original URL.
    long_url: String,
}
