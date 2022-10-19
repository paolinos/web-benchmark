import { FastifyReply, FastifyRequest } from "fastify"



export const getHealth = async (_request:FastifyRequest, _reply:FastifyReply) => {
    return { "status": "Service alive" };
}