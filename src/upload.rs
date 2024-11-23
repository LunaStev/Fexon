use actix_multipart::Multipart;
use actix_web::{HttpResponse, Result};
use futures::stream::StreamExt;
use rand::Rng;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const ALLOWED_MIME_TYPES: [&str; 2] = ["image/jpeg", "image/png"];

fn generate_unique_filename() -> String {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = duration.as_secs();

    let mut rng = rand::thread_rng();
    let random_suffix: u64 = rng.gen_range(1000..9999);

    format!("{}_{:04}", timestamp, random_suffix)
}

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
        let file_name = generate_unique_filename();
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
