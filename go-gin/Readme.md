# Go Gin
Golang + [Gin](https://gin-gonic.com/)


### Dev

```bash
# Linux/Unix
docker container run -it --rm -p 3000:3000 --network=dev-network --net-alias api -w /app -v $(pwd):/app --name go-gin-dev golang:1.19.2-alpine3.16

# Windows
docker container run -it --rm -p 3000:3000 --network=dev-network --net-alias api -w /app -v ${PWD}:/app --name go-gin-dev golang:1.19.2-alpine3.16


go ru .
```