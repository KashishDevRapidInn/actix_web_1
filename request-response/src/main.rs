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



use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/")]
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj))
}

#[get("/")]
async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(index_manual)
        .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
