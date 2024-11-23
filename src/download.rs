use actix_web::{web, HttpResponse};
use std::fs::File;
use std::path::{Path, PathBuf};
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;

/// 파일 다운로드 기능을 담당하는 함수
pub async fn download_file(file_name: web::Path<String>) -> HttpResponse {
    // 다운로드할 파일 경로
    let file_path = Path::new("uploads").join(file_name.into_inner());
    if !file_path.exists() {
        return HttpResponse::NotFound().body("File not found");
    }

    // 비동기 파일 열기
    let mut file = match OpenOptions::new().read(true).open(file_path).await {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().body("Unable to open file"),
    };

    // 파일 내용 읽기
    let mut contents = vec![];
    if let Err(_) = file.read_to_end(&mut contents).await {
        return HttpResponse::InternalServerError().body("Failed to read file");
    }

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(contents)
}
