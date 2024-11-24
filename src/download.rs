use actix_web::{web, HttpResponse};
use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;

/// Handles file download requests.
///
/// This function takes the file name from the URL path, checks if the file exists on
/// the server, and serves it as a downloadable file. If the file cannot be found or
/// there is an issue with reading it, an appropriate error response is returned.
pub async fn download_file(file_name: web::Path<String>) -> HttpResponse {
    // Construct the file path by joining the "uploads" directory with the requested file name
    let file_path = Path::new("uploads").join(file_name.into_inner());

    // Check if the file exists at the specified path
    if !file_path.exists() {
        // Return a 404 Not Found response if the file does not exist
        return HttpResponse::NotFound().body("File not found");
    }

    // Attempt to open the file for reading
    let mut file = match OpenOptions::new().read(true).open(file_path).await {
        Ok(file) => file, // Successfully opened the file
        Err(_) => {
            // Return a 500 Internal Server Error if the file cannot be opened
            return HttpResponse::InternalServerError().body("Unable to open file");
        }
    };

    // Read the entire file content into memory
    let mut contents = vec![];
    if let Err(_) = file.read_to_end(&mut contents).await {
        // Return a 500 Internal Server Error if there is an issue reading the file
        return HttpResponse::InternalServerError().body("Failed to read file");
    }

    // Return the file contents as an HTTP response with the "application/octet-stream"
    // content type, indicating it is a binary file.
    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(contents) // Send the file content as the response body
}
