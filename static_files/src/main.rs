// use actix_files::NamedFile;
// use actix_web::{web, App, HttpRequest, HttpServer, Result};
// use std::path::PathBuf;

// async fn index(req: HttpRequest) -> Result<NamedFile> {
//     let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             // Route to handle serving files based on the filename
//             .route("/{filename:.*}", web::get().to(index))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }


use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Serve files from the "static" directory
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
