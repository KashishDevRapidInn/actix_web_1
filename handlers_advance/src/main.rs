// Custom Response

// use actix_web::{web, App, HttpServer, HttpResponse, Responder,HttpRequest, body::BoxBody, http::header::ContentType};
// use serde::Serialize;

// #[derive(Serialize)]
// struct CustomResponse {
//     message: String,
//     value: i32,
// }

// impl Responder for CustomResponse {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         let body = serde_json::to_string(&self).unwrap(); // Handle serialization error in production

//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(body)
//     }
// }
// async fn custom()-> impl Responder{
//     CustomResponse{
//         message: "Hey I'm kashish".to_string(),
//         value : 22
//     }
// }

// #[actix_web::main]
// async fn main()-> std::io::Result<()>{
//     HttpServer::new(
//         ||{
//             App::new()
//                 .route("/custom", web::get().to(custom))
//         }
//     )
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }


// Either

use actix_web::{web, App, Either, Error, HttpResponse, HttpServer};

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

fn is_registered() -> bool {
    true
}

async fn index() -> RegisterResult {
    if !is_registered() {
        Either::Left(HttpResponse::BadRequest().body("Not Registered"))
    } else {
        Either::Right(Ok("Welcome!"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}