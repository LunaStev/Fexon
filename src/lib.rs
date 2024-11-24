// Declare the modules that will be part of the project
pub mod upload;  // Module responsible for handling file uploads
pub mod download; // Module responsible for handling file downloads
pub mod server;   // Module responsible for starting the server

// Re-export the functions from each module to make them accessible at the crate root
pub use upload::upload_file;  // Re-export the `upload_file` function from the `upload` module
pub use download::download_file; // Re-export the `download_file` function from the `download` module
pub use server::start_server;   // Re-export the `start_server` function from the `server` module
