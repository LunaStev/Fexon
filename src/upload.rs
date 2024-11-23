use actix_multipart::Multipart;
use actix_web::{HttpResponse, Result};
use futures::stream::StreamExt;
use futures::SinkExt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const ALLOWED_MIME_TYPES: [&str; 2] = ["image/jpeg", "image/png"];

pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
    let temp_dir = "uploads";
    create_dir_all(temp_dir).unwrap();

    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_type = field.content_type().to_string();
        if !ALLOWED_MIME_TYPES.contains(&content_type.as_str()) {
            return Ok(HttpResponse::BadRequest().body("Invalid file type"));
        }

        let mut total_size = 0;
        let file_name = Uuid::new_v4().to_string();
        let mut file_path = PathBuf::from(temp_dir);
        file_path.push(file_name);

        let mut file = File::create(file_path.clone())?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            total_size += data.len() as u64;
            if total_size > MAX_FILE_SIZE {
                return Ok(HttpResponse::BadRequest().body("File size exceeds limit"));
            }
            file.write_all(&data)?;
        }
    }

    Ok(HttpResponse::Ok().json("File uploaded successfully"))
}
