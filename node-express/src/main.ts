import express from 'express';

import { getHealth } from './routes/health';
import {
  createNote,
  getNotes,
  getNoteById
} from './routes/note';

const port = 3000;

const server = express();
server.use(express.json());


server.get('/health', getHealth);

server.post("/note", createNote)
server.get("/note", getNotes)
server.get("/note/:id", getNoteById)


server.listen(port, () => {
  console.log(`node-express runnning at ${port}`)
});