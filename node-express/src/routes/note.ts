import { 
    randomUUID 
} from "crypto";

//-----------------------------------------------------------
//														Models

type Note = {
	title: string;
	content: string;
	createdAt: Date
}

type UserNote = {
	notes:Record<string, Note>
}

//-----------------------------------------------------------
//														Declarations

const userNotes:Record<string, UserNote> = {};

export const createNote = async (req:any, res:any) => {
    // TODO: fake auth
    const {userid} = req.headers as any;
    if(!userid){
        return res.status(401).send("Invalid properties");
    }

    const {title, content} = req.body as any;
    if(!title || !content){
        return res.status(400).send("Invalid properties");
    }

    let userNote = userNotes[userid];
    if(!userNote){
        userNote = {
            notes: {}
        };
        userNotes[userid] = userNote;
    }

    const noteId = randomUUID();
    userNote.notes[noteId] = {
        title,
        content,
        createdAt: new Date()
    };

    res.status(201).json({ "id": noteId });
}

export const getNotes = async (req:any, res:any) => {
    // TODO: fake auth
    const {userid} = req.headers as any;
    if(!userid){
        return res.status(401).send("Invalid properties");
    }

    const userNote = userNotes[userid];
    if(!userNote){
        return res.json({"items":[]});
    }

    res.json({"items":userNote.notes});
}

export const getNoteById = async (req:any, res:any) => {
    // TODO: fake auth
    const {userid} = req.headers as any;
    if(!userid){
        return res.status(401).send("Invalid properties");
    }

    const { id } = req.params as any;

    const userNote = userNotes[userid];
    if(!userNote){
        return res.status(404).send("");
    }

    const note = userNote.notes[id];
    if(!note){
        return res.status(404).send("");
    }

    res.json({"item": note});
}
