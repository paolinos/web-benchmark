use std::{sync::Mutex, collections::HashMap};

use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Serialize, Deserialize};
use serde_json::json;
use uuid::Uuid;

use crate::models::model::{NoteData, Note, UserNoteData};


#[derive(Debug, Deserialize)]
pub struct InputNote {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Clone)]
pub struct NoteOutput {
    pub items: HashMap<String, Note>,
}

#[derive(Deserialize)]
pub struct NoteIdInput {
    id: String,
}

fn get_userid_header<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("userid")?.to_str().ok()
}


pub async fn create_note(users: web::Data<Mutex<UserNoteData>>, req: HttpRequest, item: web::Json<InputNote>)-> HttpResponse {
    let user_id_header = get_userid_header(&req);
    if user_id_header.is_none() {
        return HttpResponse::Unauthorized().body("");
    }

    let user_id = user_id_header.unwrap();

    let mut data = users.lock().unwrap();
    let note_data_option = data.users.get(user_id);
    

    let id = Uuid::new_v4().to_string();
    let note = Note {
        title: item.title.clone(),
        content: item.content.clone(),
    };

    if note_data_option.is_none() {
        let notes = Mutex::new(HashMap::new());
        notes.lock().unwrap().insert(id.clone(), note);

        let note_data = NoteData{
            notes: notes
        };

        data.users.insert(user_id.to_string(), note_data);
    } else {
        note_data_option.unwrap().notes.lock().unwrap().insert(id.clone(), note);
    }

    let result = json!({
        "id": id,
    });

    HttpResponse::Created().json(web::Json(result))
}

pub async fn  get_notes(users: web::Data<Mutex<UserNoteData>>, req: HttpRequest)-> HttpResponse {
    let user_id_header = get_userid_header(&req);
    if user_id_header.is_none() {
        return HttpResponse::Unauthorized().body("");
    }
    let user_id = user_id_header.unwrap();

    let data = users.lock().unwrap();
    let note_data = data.users.get(user_id);

    let result:NoteOutput;
    if note_data.is_none() {
        result = NoteOutput {
            items: HashMap::new()
        };
    }else {
        result = NoteOutput {
            items: note_data.unwrap().notes.lock().unwrap().clone()
        };
    }
    
    HttpResponse::Ok().json(result)
}

pub async fn get_note_by_id(users: web::Data<Mutex<UserNoteData>>, req: HttpRequest, note_input: web::Path<NoteIdInput>,)-> HttpResponse {
    let user_id_header = get_userid_header(&req);
    if user_id_header.is_none() {
        return HttpResponse::Unauthorized().body("");
    }
    let user_id = user_id_header.unwrap();

    let data = users.lock().unwrap();
    let note_data = data.users.get(user_id);
    if note_data.is_none() {
        return HttpResponse::NotFound().body("");
    }
    

    let note_data = note_data.unwrap();
    let notes = note_data.notes.lock().unwrap();
    let note = notes.get(&note_input.id);
    
    if note.is_none() {
        return HttpResponse::NotFound().body("");
    }
    
    let result = json!({
        "item": note, 
    });

    HttpResponse::Ok().json(result)
}