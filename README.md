# Windrop - File Upload Service with Rate Limiting and Logging Middleware

## Overview

Windrop is an Actix Web-based file upload service that allows users to upload files with rate limiting and request logging. The service integrates several key features:

- **File Upload**: Handles multipart file uploads and stores them on disk.
- **File Retrieval**: Retrieves and streams files from disk, allowing users to download files by their ID.
- **Rate Limiting**: Protects the server from abuse by limiting the number of requests per client IP.
- **Request Logging**: Logs HTTP request details including status codes and response times.

This project aims to provide a simple and effective file upload service with essential protections for handling large traffic volumes.

## Features

- **Rate Limiting**: Limits the number of requests per client IP within a given time window (e.g., 100 requests per minute).
- **File Upload**: Allows users to upload files, which are stored in a configurable storage directory.
- **File Download**: Users can retrieve files using a unique file ID.
- **Logging**: Logs all incoming HTTP requests, including method, path, status code, and response time.
- **Error Handling**: Returns appropriate error responses when upload or file retrieval fails.

## Installation

To install and run this project locally, follow the steps below:

### Prerequisites

- Rust (1.70.0 or later)
- Actix Web (4.0 or later)
- Tokio
- Actix Multipart

### Steps

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/windrop.git
    cd windrop
    ```

2. Build the project:

    ```bash
    cargo build
    ```

3. Run the server:

    ```bash
    cargo run
    ```

    The server will start at `http://127.0.0.1:8080`.

4. You can now test the rate-limiting and file upload functionality by making requests to the `/upload` and `/files/{id}` endpoints.

## Configuration

You can customize the rate-limiting behavior by adjusting the `max_requests` and `window_duration` when creating the `RateLimiter`.

- **`max_requests`**: The maximum number of requests allowed per client IP within the time window.
- **`window_duration`**: The duration of the time window in which requests are counted (e.g., 1 minute, 10 seconds).

### Example Rate Limiting Configuration

```rust
let rate_limiter = RateLimiter::new(100, Duration::from_secs(60));  // 100 requests per minute
