import Fastify, { FastifyInstance } from 'fastify';
import { getHealth } from './routes/health';
import {
  createNote,
  getNotes,
  getNoteById
} from './routes/note';

const server: FastifyInstance = Fastify({
  logger: true
})

server.get('/health', getHealth);

server.post("/note", createNote)
server.get("/note", getNotes)
server.get("/note/:id", getNoteById)

/*
const start = async () => {
  try {
    await server.listen({ host:"0.0.0.0", port: 3000 })
  } catch (err) {
    server.log.error(err)
    process.exit(1)
  }
}
start()
*/
server.listen(3000, '0.0.0.0');