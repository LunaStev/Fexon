# Fexon
Fexon is a simple file upload and download server written in Rust. It uses the Actix framework to provide functionality for uploading and downloading files over HTTP. It offers a fast and secure file management server solution.

## Features
* File upload and download functionality
* Multipart form-data support for file uploads
* Simple API for project integration
* High performance implementation with Actix and async Rust
* Easy setup and deployment

## Installation
To include `fexon` in your Rust project, add it to your `Cargo.toml` file as follows:

```toml
[dependencies]
fexon = "0.1.0"
```

## Example Usage
Below is an example code to upload and download files using `fexon`.

### Server Example
```rust
use fexon::{start_server, upload_file, download_file};
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
HttpServer::new(|| {
App::new()
.route("/upload", web::post().to(upload_file))
.route("/download/{file_name}", web::get().to(download_file))
})
.bind("127.0.0.1:8080")?
.run()
.await
}
```

### File Upload Example
To upload a file, send a `POST` request to the `/upload` endpoint with the file as `multipart/form-data`.

### File Download Example
To download a file, send a GET request to the /download/{file_name} endpoint.

## API Endpoints
* **POST** `/upload`: Upload a file.

    * **Request**: Send the file as `multipart/form-data`.
    * **Response**: Returns a success message or an error message.

* GET /download/{file_name}: Download a file by its name.

    * Request: Provide the file name in the URL path.
    * Response: Returns the requested file.

## Requirements
* Rust 1.50 or later
* Actix framework (4.x)
* Tokio runtime (1.x)

# License
This project is licensed under the Mozilla Public License 2.0 (MPL-2.0). Please refer to the [LICENSE](LICENSE) file for more details.

# Contributing
If you'd like to contribute to fexon, feel free to fork the repository and create a pull request. Contributions are always welcome!

1. Fork the repository.
2. Create a new branch (git checkout -b feature-name).
3. Commit your changes (git commit -am 'Add feature').
4. Push the branch (git push origin feature-name).
5. Create a new pull request.
