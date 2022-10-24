use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ServiceStatus {
    status: String,
}


#[get("/")]
pub fn get_health() -> Json<ServiceStatus> {
    let result = ServiceStatus {
        status: "Service alive".to_string(),
    };

    Json(result)
}