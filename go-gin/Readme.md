# Go Gin
This part is still in development.
Some issues with concurrency that could be related with map or with gin

### Dev

```bash
# Linux/Unix
docker container run -it --rm -p 3000:3000 --network=dev-network --net-alias api -w /app -v $(pwd):/app --name go-gin-dev golang:1.19.2-alpine3.16

# Windows
docker container run -it --rm -p 3000:3000 --network=dev-network --net-alias api -w /app -v ${PWD}:/app --name go-gin-dev golang:1.19.2-alpine3.16


go run .
```

**locust**
```bash
# in the root (..) run:
docker container run --rm --network=host --name locust -w /locust -v $(pwd)/locust:/locust locustio/locust -f go-gin.py --headless --users 1000 --spawn-rate 100 -t 10s -H http://localhost:3000

docker container run --rm --network=host --name locust -w /locust -v ${PWD}/locust:/locust locustio/locust -f go-gin.py --headless --users 1000 --spawn-rate 100 -t 10s -H http://localhost:3000


# Issue? maybe gin framework?
goroutine 357 [runnable]:
net/http.(*Server).Serve.func3()
        /usr/local/go/src/net/http/server.go:3102
runtime.goexit()
        /usr/local/go/src/runtime/asm_amd64.s:1594 +0x1
created by net/http.(*Server).Serve
        /usr/local/go/src/net/http/server.go:3102 +0x4db
```