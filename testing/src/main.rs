use actix_web::{web, App, HttpServer, HttpResponse, Responder, test, http::header::ContentType};
use serde::{Serialize, Deserialize};
use serde_json::json;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Serialize,Deserialize)]
struct Person {
    name: String,
    age: u32,
}

async fn create(item: web::Json<Person>) -> impl Responder {
    HttpResponse::Ok().json(item.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


#[cfg(test)]
mod tests{
    use super::*;
    #[actix_web::test]
    async fn test_index(){
        // Initialize the application for testing
          let app = test::init_service(App::new().route("/", web::get().to(index))).await;
        // Create a GET request with default settings
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        
        // Call the service with the request
        let resp = test::call_service(&app, req).await;
        
        // Assert the response status is successful
        assert!(resp.status().is_success());
    }

     #[actix_web::test]
    async fn test_create(){
        let app = test::init_service(App::new().route("/create", web::post().to(create))).await;

        let payload= json!({
            "name": "kk",
            "age": 22
        });

        let req = test::TestRequest::post()
            .uri("/create")
            .insert_header(ContentType::json())
            .set_payload(payload.to_string())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let expected_response = json!({
            "name": "kk",
            "age": 22
        });
        let body = test::read_body(resp).await;
        let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(response_json, expected_response);
    }
}   