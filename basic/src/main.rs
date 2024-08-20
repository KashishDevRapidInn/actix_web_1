use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
/**************/
/**  State   **/
/**************/
struct AppState {
    app_name: String,
}


/**************/
/** Handlers **/
/**************/

#[get("/")]
async fn hello()-> impl Responder{
    HttpResponse::Ok().body("Hello World")
}

#[post("/register")]
async fn register(req_body: String)-> impl Responder{
    HttpResponse::Ok().body(req_body)
}
    
async fn manual_handler()-> impl Responder{
    HttpResponse::Ok().body("Manual Handler")
}

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(hello)
            .service(register)
            .route("/hey", web::get().to(manual_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
