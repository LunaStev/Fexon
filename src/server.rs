use actix_web::{web, App, HttpServer};
use crate::{upload_file, download_file};

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_file))
            .route("/download/{file_name}", web::get().to(download_file))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
