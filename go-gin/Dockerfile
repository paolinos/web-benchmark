# Build project
FROM golang:1.19.2-alpine3.16 AS build-project
WORKDIR /app

COPY ./go.mod ./
COPY ./go.sum ./
COPY ./model ./model
COPY ./routes ./routes
COPY ./main.go ./

RUN go build

# Production image
FROM golang:1.19.2-alpine3.16
WORKDIR /app
COPY --from=build-project /app/go-gin .

EXPOSE 3000

CMD ["./go-gin"]