// // use actix_web::{get, web, App, HttpServer, Result};

// // /// extract path info from "/users/{user_id}/{friend}" url
// // /// {user_id} - deserializes to a u32
// // /// {friend} - deserializes to a String
// // #[get("/users/{user_id}/{friend}")] // <- define path parameters
// // async fn index(path: web::Path<(u32, String)>) -> Result<String> {
// //     let (user_id, friend) = path.into_inner();
// //     Ok(format!("Welcome {}, user_id {}!", friend, user_id))
// // }

// // #[actix_web::main]
// // async fn main() -> std::io::Result<()> {
// //     HttpServer::new(|| App::new().service(index))
// //         .bind(("127.0.0.1", 8080))?
// //         .run()
// //         .await
// // }


// use actix_web::{get, web, Result, HttpResponse, Responder};
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct Info {
//     user_id: u32,
//     friend: String,
// }

// /// extract path info using serde
// // #[get("/users/{user_id}/{friend}")] // <- define path parameters
// // async fn index(info: web::Path<Info>) -> Result<String> {
// //     Ok(format!(
// //         "Welcome {}, user_id {}!",
// //         info.friend, info.user_id
// //     ))
// // }

// /// extract query info using serde
// async fn info_query(info: web::Query<Info>)->String{
//     format!("Hello {} {}", info.user_id, info.friend)
// }


// /// Json
// #[derive(Deserialize)]
// struct Person {
//     name: String,
//     age: u32,
// } 
// async fn create_person(person: web::Json<Person>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Received person: {} (age {})", person.name, person.age))
// }

// /// From Data
// #[derive(Deserialize)]
// struct FormData {
//     username: String,
//     password: String,
// }

// async fn login(form: web::Form<FormData>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Username: {}, Password: {}", form.username, form.password))
// }




// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};

//     // HttpServer::new(|| App::new().service(info_query))
//     //     .bind(("127.0.0.1", 8080))?
//     //     .run()
//     //     .await
//      HttpServer::new(|| {
//         App::new()
//             .route("/info", web::get().to(info_query))
//             .route("/person", web::post().to(create_person))
//             .route("/login", web::post().to(login))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }



//Application State Extractor
use actix_web::{web, App, HttpServer, Responder};
use std::cell::Cell;

#[derive(Clone)]
struct AppState {
    count: Cell<usize>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.get())
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("count: {}", data.count.get())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        count: Cell::new(0),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .route("/", web::to(show_count))
            .route("/add", web::to(add_one))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
