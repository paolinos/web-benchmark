use std::collections::HashMap;
use std::sync::Mutex;
mod models;
mod routes;
use models::model::{UserNoteData};


#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let users_data:Mutex<UserNoteData> = Mutex::new(UserNoteData{
        users: HashMap::new()
    });

    let _rocket = rocket::build()
        .manage(users_data)
        .mount("/health", routes![routes::health::get_health])
        .mount("/note", routes![
            routes::note::create_note, 
            routes::note::get_note_all, 
            routes::note::get_note_id
        ])
        .launch()
        .await?;

    Ok(())
}