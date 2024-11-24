use actix_web::{web, App, HttpServer};
use crate::{upload_file, download_file};

/// Starts the HTTP server and defines the routes for file upload and download.
///
/// This function sets up an Actix web server that listens on `127.0.0.1:8080` and
/// exposes two routes: `/upload` for handling file uploads (via the `upload_file`
/// function) and `/download/{file_name}` for handling file downloads (via the
/// `download_file` function). The server will respond to these routes asynchronously.
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        // Create a new Actix web application with routes defined
        App::new()
            // Define the "/upload" POST route that maps to the `upload_file` handler
            .route("/upload", web::post().to(upload_file))
            // Define the "/download/{file_name}" GET route that maps to the `download_file` handler
            .route("/download/{file_name}", web::get().to(download_file))
    })
        // Bind the server to the specified address (127.0.0.1:8080) and run it
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
