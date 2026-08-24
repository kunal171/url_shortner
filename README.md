# URL Shortener

A small Rust URL shortener built with Axum, Tokio, and MongoDB.

The project focuses on backend fundamentals: HTTP routing, JSON APIs, input
validation, MongoDB persistence, redirects, and simple analytics.

## Features

- Create short links for valid `http` and `https` URLs
- Redirect short codes to the original long URL
- Track visit counts
- Fetch URL stats by short code
- Generate collision-checked 7-character short codes
- Return consistent JSON errors
- Load configuration from environment variables

## Tech Stack

- Rust 2024 edition
- Tokio async runtime
- Axum web framework
- MongoDB Rust driver
- Serde for JSON serialization
- dotenvy for local environment configuration

## Project Structure

```text
src/
  main.rs       Server setup, routes, shared state
  api.rs        HTTP handlers
  database.rs   MongoDB connection setup
  url.rs        URL models, validation, short-code generation
  error.rs      Shared JSON error response type
  lib.rs        Library module exports
```

## API

### Health Check

```bash
curl http://127.0.0.1:3001/health
```

Expected response:

```text
ok
```

### Create Short URL

```bash
curl -X POST http://127.0.0.1:3001/api/shorten \
  -H 'content-type: application/json' \
  -d '{"long_url":"https://www.google.com"}'
```

Example response:

```json
{
  "code": "xp3uvKQ",
  "short_url": "http://127.0.0.1:3001/xp3uvKQ",
  "long_url": "https://www.google.com"
}
```

### Redirect

```bash
curl -i http://127.0.0.1:3001/xp3uvKQ
```

The server returns `302 Found` with a `Location` header pointing to the original
URL.

### URL Stats

```bash
curl http://127.0.0.1:3001/api/urls/xp3uvKQ/stats
```

Example response:

```json
{
  "code": "xp3uvKQ",
  "long_url": "https://www.google.com",
  "short_url": "http://127.0.0.1:3001/xp3uvKQ",
  "visits": 1,
  "created_at": {
    "$date": "2026-08-24T10:30:00Z"
  }
}
```

## Configuration

Create a local `.env` file:

```env
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=url_shortner
BASE_URL=http://127.0.0.1:3001
```

`BASE_URL` should match the host and port where the server is reachable.

## Run Locally

Start MongoDB, then run:

```bash
cargo run
```

The server listens on:

```text
http://127.0.0.1:3001
```

## Notes

This is a learning-focused backend project. Good next improvements would be a
unique MongoDB index on `code`, integration tests, configurable bind address,
and Docker Compose for local MongoDB.
