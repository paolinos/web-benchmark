import { 
    randomUUID 
} from "crypto";
import { 
    FastifyReply, 
    FastifyRequest 
} from "fastify"

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

export const createNote = async (request:FastifyRequest, reply:FastifyReply) => {
    // TODO: fake auth
    const {userid} = request.headers as any;
    if(!userid){
        reply.status(401); 
        return "Invalid properties"
    }

    const {title, content} = request.body as any;
    if(!title || !content){
        reply.status(400); 
        return "Invalid properties"
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

    reply.status(201);
    return { "id": noteId };
}

export const getNotes = async (request:FastifyRequest, reply:FastifyReply) => {
    // TODO: fake auth
    const {userid} = request.headers as any;
    if(!userid){
        reply.status(401); 
        return "Invalid properties"
    }

    const userNote = userNotes[userid];
    if(!userNote){
        return {"items":[]};
    }

    return {"items": userNote.notes};
}

export const getNoteById = async (request:FastifyRequest, reply:FastifyReply) => {
    // TODO: fake auth
    const {userid} = request.headers as any;
    if(!userid){
        reply.status(401); 
        return "Invalid properties"
    }

    const { id } = request.params as any;

    const userNote = userNotes[userid];
    if(!userNote){
        reply.status(404); 
        return ""
    }

    const note = userNote.notes[id];
    if(!note){
        reply.status(404); 
        return ""
    }

    return {"item": note};
}