use std::{sync::Mutex, collections::HashMap};
use rocket::{
    http::Status, 
    serde::json::Json, 
    State, 
    response::status::Created,
    request,
    request::Request, 
    request::FromRequest
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::model::{NoteData, Note, UserNoteData};

#[derive(Deserialize)]
pub struct InputNote {
    title: String,
    content: String
}
#[derive(Serialize)]
pub struct OutputId {
    id: String,
}

#[derive(Serialize, Clone)]
pub struct NoteOutput {
    pub items: HashMap<String, Note>,
}


pub struct UserIdHeader(String);

#[derive(Debug)]
pub enum UserIdHeaderError {
    NotFound,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserIdHeader {
    type Error = UserIdHeaderError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<UserIdHeader, Self::Error> {
        let value = req.headers().get_one("userid");
        if value.is_none(){
            return request::Outcome::Failure((Status::Unauthorized, UserIdHeaderError::NotFound));
        }

        request::Outcome::Success(UserIdHeader(value.unwrap().to_string()))
    }
}


#[get("/")]
pub fn get_note_all(users: &State<Mutex<UserNoteData>>, useridheader: UserIdHeader) -> Result<Json<NoteOutput>, Status> {
    
    let data = users.lock().unwrap();
    let note_data = data.users.get(&useridheader.0);

    let result:NoteOutput;
    if note_data.is_none() {
        result= NoteOutput{ 
            items: HashMap::new() 
        };
    }else{
        result = NoteOutput{
            items: note_data.unwrap().notes.lock().unwrap().clone()
        };
    }

    /*
    //let note_data = note_state.lock().unwrap();
    
    let result = NoteOutput{
        items: note_data.unwrap().notes.lock().unwrap().clone()
    };
    */

    Ok(Json(result))
}

#[get("/<id>")]
pub fn get_note_id(users: &State<Mutex<UserNoteData>>, useridheader: UserIdHeader,id: &str) -> Result<Json<Note>, Status> {
    let data = users.lock().unwrap();
    let note_data = data.users.get(&useridheader.0);
    if note_data.is_none() {
        return Err(Status::NotFound);
    }

    let note_data = note_data.unwrap();
    let notes = note_data.notes.lock().unwrap();
    let note = notes.get(id);


    // NOTE: match didn't work ....
    if note.is_none() {
        return Err(Status::NotFound);
    }
    
    Ok(Json(note.unwrap().clone()))
}

#[post("/", format = "json", data="<new_note>")]
pub fn create_note(users: &State<Mutex<UserNoteData>>, useridheader: UserIdHeader, new_note: Json<InputNote>) -> Created<Json<OutputId>> {
    let mut data = users.lock().unwrap();
    let note_data_option = data.users.get(&useridheader.0);
    

    let id = Uuid::new_v4().to_string();
    let note = Note {
        title: new_note.title.clone(),
        content: new_note.content.clone()
    };
    //notes.insert(id.clone(), note);

    if note_data_option.is_none() {

        let notes = Mutex::new(HashMap::new());
        notes.lock().unwrap().insert(id.clone(), note);

        let note_data = NoteData{
            notes: notes
        };
        
        //let mut asdasd = users.lock().unwrap();
        //note_data_option.
        data.users.insert(useridheader.0, note_data);
        //.insert(useridheader.0, note_data);

        
    }else {
        note_data_option.unwrap().notes.lock().unwrap().insert(id.clone(), note);
    }

    

    
    
    //let mut note_data = note_state.unwrap();
    
    
    
    

    let url = format!("/note/{}", id.clone());
    let result = OutputId {
        id: id,
    };

    Created::new(url).body(Json(result))
}
