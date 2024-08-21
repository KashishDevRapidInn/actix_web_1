// use actix_web::{App, HttpServer};
// use actix_web::middleware::Logger;
// use env_logger::Env;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init_from_env(Env::default().default_filter_or("info"));

//     HttpServer::new(|| {
//         App::new()
//             .wrap(Logger::default()) // Use default logging format
//             // .wrap(Logger::new("%a %{User-Agent}i")) // Custom logging format
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }



// use actix_web::{web, App, HttpResponse, HttpServer, middleware};
// use actix_web::http::Method;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             // Add DefaultHeaders middleware to set a custom header for all responses
//             .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
//             .service(
//                 web::resource("/test")
//                     .route(web::get().to(HttpResponse::Ok))
//                     .route(web::method(Method::HEAD).to(HttpResponse::MethodNotAllowed)),
//             )
//     })
//     .bind("127.0.0.1:8080")? // Bind to localhost:8080
//     .run()
//     .await
// }


