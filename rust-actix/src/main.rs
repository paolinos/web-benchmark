use std::{collections::HashMap, sync::Mutex};

use actix_web::{web, App, HttpServer, middleware::Logger};
use models::model::UserNoteData;
use routes::note;
mod routes;
mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let users_data:Mutex<UserNoteData> = Mutex::new(UserNoteData{
        users: HashMap::new()
    });
    let data_actix = web::Data::new(users_data);

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(web::Data::clone(&data_actix))
            .service(routes::health::get_health)
            .route("/note", web::get().to(note::get_notes))
            .route("/note", web::post().to(note::create_note))
            .route("/note/{id}", web::get().to(note::get_note_by_id))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}