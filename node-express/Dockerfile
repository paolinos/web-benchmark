# Build project
FROM node:alpine3.16 AS build-project
WORKDIR /app

COPY ./package.json ./
COPY ./package-lock.json ./
RUN npm ci

COPY ./src ./src
COPY ./tsconfig.json ./

ENV NODE_ENV production
RUN npm run build

# Production image
FROM node:alpine3.16
WORKDIR /app
COPY --from=build-project /app/dist .
COPY --from=build-project /app/package.json .
COPY --from=build-project /app/package-lock.json .
RUN npm ci --production
EXPOSE 3000
CMD ["node","main.js"]