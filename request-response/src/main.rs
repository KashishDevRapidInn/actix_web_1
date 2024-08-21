/****************************/
/****     Requests     ******/
/****************************/

// use actix_web::{web, App, HttpServer, Result};
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct Info {
//     username: String,
// }

// /// extract `Info` using serde
// async fn index(info: web::Json<Info>) -> Result<String> {
//     Ok(format!("Welcome {}!", info.username))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/", web::post().to(index)))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }



// use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer};
// use futures::StreamExt;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct MyObj {
//     name: String,
//     number: i32,
// }

// const MAX_SIZE: usize = 262_144; // max payload size is 256k

// #[post("/")]
// async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
//     let mut body = web::BytesMut::new();
//     while let Some(chunk) = payload.next().await {
//         let chunk = chunk?;
//         if (body.len() + chunk.len()) > MAX_SIZE {
//             return Err(error::ErrorBadRequest("overflow"));
//         }
//         body.extend_from_slice(&chunk);
//     }

//     let obj = serde_json::from_slice::<MyObj>(&body)?;
//     Ok(HttpResponse::Ok().json(obj))
// }

// #[get("/")]
// async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
//     let mut bytes = web::BytesMut::new();
//     while let Some(item) = body.next().await {
//         let item = item?;
//         println!("Chunk: {:?}", &item);
//         bytes.extend_from_slice(&item);
//     }

//     Ok(HttpResponse::Ok().finish())
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//         .service(index_manual)
//         .service(index)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }


/****************************/
/****     Responses    ******/
/****************************/

// use actix_web::{web, get, App, HttpResponse,HttpServer, Responder, Result};
// use serde::{Serialize,Deserialize};

// #[derive(Serialize,Deserialize)]
// struct Info{
//     name: String
// }
// #[get("/{name}")]
// async fn index(info: web::Path<Info>)-> Result<impl Responder>{
//     let obj= Info{
//         name: info.name.to_string(),
//     };
//     Ok(web::Json(obj)) //serialization
// }

// #[actix_web::main]
// async fn main()-> std::io::Result<()>{
//     HttpServer::new(||{
//         App::new()
//             .service(index)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }



// Content-Encoding(Auto-Compression)
// use actix_web::{get, middleware, App, HttpResponse, HttpServer};

// #[get("/")]
// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("data")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::Compress::default())
//             .service(index)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// Explicitly Disabling Compression
// use actix_web::{
//     get, http::header::ContentEncoding, middleware, App, HttpResponse, HttpServer,
// };

// #[get("/")]
// async fn index() -> HttpResponse {
//     HttpResponse::Ok()
//         .insert_header(ContentEncoding::Identity) // disables compression for some routes even if compression middleware is used
//         .body("data")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::Compress::default())
//             .service(index)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }


// use actix_web::{
//     get, http::header::ContentEncoding, middleware, App, HttpResponse, HttpServer,
// };

// static HELLO_WORLD: &[u8] = &[
//     0x1f, 0x8b, 0x08, 0x00, 0xa2, 0x30, 0x10, 0x5c, 0x00, 0x03, 0xcb, 0x48, 0xcd, 0xc9, 0xc9,
//     0x57, 0x28, 0xcf, 0x2f, 0xca, 0x49, 0xe1, 0x02, 0x00, 0x2d, 0x3b, 0x08, 0xaf, 0x0c, 0x00,
//     0x00, 0x00,
// ];

// #[get("/")]
// async fn index() -> HttpResponse {
//     HttpResponse::Ok()
//         .insert_header(ContentEncoding::Gzip)
//         .body(HELLO_WORLD)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::Compress::default())
//             .service(index)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
