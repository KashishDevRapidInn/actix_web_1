/***********************/
/*****Custom Error******/
/***********************/


// use actix_web::{web, App, HttpServer, error, Result};
// use derive_more::derive::{Display, Error};

// #[derive(Debug, Display, Error)]
// #[display("my error: {name}")]
// struct MyError {
//     name: &'static str,
// }


// impl error::ResponseError for MyError {}

// async fn index() -> Result<&'static str, MyError> {
//     Err(MyError { name: "test" })
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(index))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }


// use actix_web::{web, App, error, get,http::{header::ContentType, StatusCode}, HttpResponse, HttpServer};
// use derive_more::derive::{Display, Error};

// #[derive(Debug, Display, Error)]
// enum MyError {
//     #[display("internal error")]
//     InternalError,

//     #[display("bad request")]
//     BadClientData,

//     #[display("timeout")]
//     Timeout,
// }

// impl error::ResponseError for MyError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::html())
//             .body(self.to_string())
//     }

//     fn status_code(&self) -> StatusCode {
//         match *self {
//             MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//             MyError::BadClientData => StatusCode::BAD_REQUEST,
//             MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
//         }
//     }
// }

// #[get("/")]
// async fn index() -> Result<&'static str, MyError> {
//     Err(MyError::BadClientData)
// }
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(index)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }



/***********************/
/**** Error helpers ****/
/***********************/

// use actix_web::{error, get, App, HttpServer};

// #[derive(Debug)]
// struct MyError {
//     name: &'static str,
// }

// #[get("/")]
// async fn index() -> actix_web::Result<String> {
//     let result = Err(MyError { name: "test error" });

//     result.map_err(|err| error::ErrorBadRequest(err.name))
// }
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(index)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }



/**************************/
/***** Error Mapping  *****/
/**************************/
use actix_web::{error, get, http::{header::ContentType, StatusCode}, App, HttpResponse, HttpServer,};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
enum UserError {
    #[display("An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
fn do_thing_that_fails() -> Result<(), String> {
    Err("Getting A Error: ".to_string())
}
#[get("/")]
async fn index() -> Result<&'static str, UserError> {
    // Simulate a failure
    do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
