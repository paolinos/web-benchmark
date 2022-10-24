use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
pub async fn get_health() -> impl Responder {
    
    let posts = json!({
        "status": "Service alive"
    });

    HttpResponse::Ok().json(web::Json(posts))
}