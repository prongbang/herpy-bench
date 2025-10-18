use actix_web::{
    delete, get, patch, post, put, web, App, HttpResponse, HttpServer, Responder, Result,
};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Listen :8000")
}

#[get("/v1/hello")]
async fn hello() -> Result<impl Responder> {
    let obj = MyObj {
        name: "Hello World!".to_string(),
    };
    Ok(web::Json(obj))
}

#[get("/get")]
async fn get() -> Result<impl Responder> {
    let obj = MyObj {
        name: "Hello World!".to_string(),
    };
    Ok(web::Json(obj))
}

#[post("/post")]
async fn post() -> Result<impl Responder> {
    let obj = MyObj {
        name: "Hello World!".to_string(),
    };
    Ok(web::Json(obj))
}

#[put("/put")]
async fn put() -> Result<impl Responder> {
    let obj = MyObj {
        name: "Hello World!".to_string(),
    };
    Ok(web::Json(obj))
}

#[patch("/patch")]
async fn patch() -> Result<impl Responder> {
    let obj = MyObj {
        name: "Hello World!".to_string(),
    };
    Ok(web::Json(obj))
}

#[delete("/delete")]
async fn delete() -> Result<impl Responder> {
    let obj = MyObj {
        name: "Hello World!".to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(get)
            .service(post)
            .service(put)
            .service(patch)
            .service(delete)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
