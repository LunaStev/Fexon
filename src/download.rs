use actix_web::{web, HttpResponse};
use std::fs::File;
use std::path::{Path, PathBuf};
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;

pub async fn download_file(file_name: web::Path<String>) -> HttpResponse {
    let file_path = Path::new("uploads").join(file_name.into_inner());
    if !file_path.exists() {
        return HttpResponse::NotFound().body("File not found");
    }

    let mut file = match OpenOptions::new().read(true).open(file_path).await {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().body("Unable to open file"),
    };

    let mut contents = vec![];
    if let Err(_) = file.read_to_end(&mut contents).await {
        return HttpResponse::InternalServerError().body("Failed to read file");
    }

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(contents)
}
