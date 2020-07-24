extern crate diesel;
extern crate uuid;
extern crate simple_rest_one;

use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Responder, Error};
use serde::Serialize;
use futures::future::{ready, Ready};
use simple_rest_one::models::Contact;
use uuid::Uuid;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{

    use simple_json_one::schema;

    HttpServer::new(|| {
        App::new().route("/", web::to(index))
    })
        .bind("0.0.0.0:6123")?
        .run()
        .await
}


impl Responder for Contact {
    type Error  = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
        ))
    }
}

async fn index() -> impl Responder {
    Contact {
        id: 0,
        uuid: Uuid::new_v4().to_string(),
        name: "Robin van Leeuwen".to_string(),
        address: "Martin Luther Kinglaan 206".to_string(),
        zipcode: "1111 LM".to_string(),
        city: "Diemen".to_string(),
        country: "Nederland".to_string()
    }
}