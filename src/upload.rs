use actix_multipart::Multipart;
use actix_web::{HttpResponse, Result};
use futures::stream::StreamExt;
use nanoid::nanoid;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

// Maximum file size allowed for upload (10MB)
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
// List of allowed MIME types for the uploaded files (JPEG, PNG)
const ALLOWED_MIME_TYPES: [&str; 3] = ["image/jpeg", "image/png", "image/jpg"];

/// Generates a unique filename based on a random ID and file extension.
///
/// This function uses `nanoid` to generate a random ID and appends the file extension to
/// create a unique filename. This ensures that filenames do not conflict when multiple
/// files are uploaded.
fn generate_unique_filename(extension: &str) -> String {
    let unique_id = nanoid!(); // Generate a unique ID using nanoid
    format!("{}_{}", unique_id, extension) // Combine the unique ID with the file extension
}

/// Handles file upload from a multipart request.
///
/// This function processes a file uploaded via `Multipart` from an HTTP request. It
/// validates the file type, checks the file size, and saves the file to a temporary
/// directory on the server. If the file exceeds the allowed size or has an invalid
/// MIME type, it returns a `BadRequest` response.
pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
    // Directory to store uploaded files
    let temp_dir = "uploads";
    // Create the directory if it doesn't exist
    create_dir_all(temp_dir).unwrap();

    // Iterate over the multipart fields and process each file
    while let Some(item) = payload.next().await {
        let mut field = item?; // Get the next multipart field

        // Get the content type (MIME type) of the file
        let content_type = field.content_type().to_string();

        // Determine the file extension based on MIME type
        let extension = match content_type.as_str() {
            "image/jpeg" => "jpg", // JPEG image
            "image/png" => "png",  // PNG image
            "image/jpg" => "jpg",  // JPG image
            _ => {
                // Return a BadRequest if the MIME type is not allowed
                return Ok(HttpResponse::BadRequest().body("Invalid file type"));
            }
        };

        // Variable to track the total file size
        let mut total_size = 0;
        // Generate a unique filename for the uploaded file
        let file_name = generate_unique_filename(extension);
        // Create the file path for saving the uploaded file
        let mut file_path = PathBuf::from(temp_dir);
        file_path.push(file_name); // Append the unique filename

        // Create a new file at the determined path
        let mut file = File::create(file_path.clone())?;

        // Process the file data chunk by chunk
        while let Some(chunk) = field.next().await {
            let data = chunk?; // Get the current chunk of data
            total_size += data.len() as u64; // Update the total file size

            // If the file size exceeds the maximum allowed, return an error
            if total_size > MAX_FILE_SIZE {
                return Ok(HttpResponse::BadRequest().body("File size exceeds limit"));
            }

            // Write the chunk of data to the file
            file.write_all(&data)?;
        }
    }

    // Return a success response once the file is uploaded successfully
    Ok(HttpResponse::Ok().json("File uploaded successfully"))
}
