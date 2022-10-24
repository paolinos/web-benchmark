use std::{collections::HashMap, sync::Mutex};

use serde::{Serialize};

#[derive(Serialize, Clone)]
pub struct Note {
    pub title: String,
    pub content: String,
}

pub struct NoteData {
    pub notes: Mutex<HashMap<String, Note>>,
}

pub struct UserNoteData {
    pub users: HashMap<String, NoteData>,
}