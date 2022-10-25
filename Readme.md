# Web benchmark

I'm trying to do a simple test, just a run some benchmark in different frameworks/langauges and all be running in Docker.  
So to run the benchmark I'm using [Locust](https://locust.io/) a python load testing tool.

The projects will have the same endpoints, you can take a look to the [open-api](./open-api.spec.yaml) specification
So for benchmark we're going to use:
- GET /health => return json
- POST /note => create a new Note and save it in the dictionary for the user (check header validation **open-api** file)
- GET /note => get all notes for the user (check header validation **open-api** file)

**Results:** You can find the results in the [Result](./Result.md) file.  
 

### Folders
- locust => Locust load test tool [Readme](./locust/Readme.md)
- node-fastify => Nodejs using Fastify framework [Readme](./node-fastify/Readme.md)
- node-express => Nodejs using Express framework [Readme](./node-express/Readme.md)
- dotnet => C# .Net Core 6 [Readme](./net-core/Readme.md)
- rust-rocket => Rust using Rocket framework [Readme](./rust-rocket/Readme.md)
- rust-actix => Rust using Actix framework [Readme](./rust-actix/Readme.md)
- go-gin -> Golang using Gin framework [Readme](./go-gin/Readme.md)

### Settings
Before to start running the different project you need to have installed [Docker](https://www.docker.com/) and then you can create:
```bash
# Network that we're going to use for the services
docker network create -d bridge dev-network
```


### Running tests
To run the test, first you need to run **One** project and then run the load test. (all projects are going to run at same port)

**Projects:**
- node-fastify:
```bash
# Build node-fastify project
docker build --no-cache --tag node-fastify ./node-fastify

# Run docker
docker container run -it --rm --network=dev-network -p 3000:3000 --net-alias api --name node-fastify node-fastify
```

- dotnet:
```bash
# Build node-fastify project
docker build --no-cache --tag net-core ./net-core

# Run docker
docker container run -it --rm --network=dev-network -p 3000:3000 --net-alias api --name net-core net-core
```

- node-express:
```bash
# Build node-fastify project
docker build --no-cache --tag node-express ./node-express

# Run docker
docker container run -it --rm --network=dev-network -p 3000:3000 --net-alias api --name node-express node-express
```

- rust-rocket:
```bash
# Build node-fastify project
docker build --no-cache --tag rust-rocket ./rust-rocket

# Run docker
docker container run -it --rm --network=dev-network -p 3000:3000 --net-alias api --name rust-rocket rust-rocket
```

- rust-actix:
```bash
# Build node-fastify project
docker build --no-cache --tag rust-actix ./rust-actix

# Run docker
docker container run -it --rm --network=dev-network -p 3000:3000 --net-alias api --name rust-actix rust-actix
```

- go-gin:
```bash
# Build node-fastify project
docker build --no-cache --tag go-gin ./go-gin

# Run docker
docker container run -it --rm --network=dev-network -p 3000:3000 --net-alias api --name go-gin go-gin
```

**Locust:**
for more info about locust settings please take a look to the [Readme](./locust/Readme.md)

```bash
# Remember docker volume
#   $(pwd) => Linux/Unix
#   ${PWD} => Powershell - Windows
docker container run --rm -p 8089:8089 --network=dev-network --name locust -w /locust -v ${PWD}/locust:/locust locustio/locust -f api-benchmark.py --headless --users 100 --spawn-rate 10 -t 60s -H http://api:3000

# Unix version
docker container run --rm -p 8089:8089 --network=dev-network --name locust -w /locust -v $(pwd)/locust:/locust locustio/locust -f api-benchmark.py --headless --users 100 --spawn-rate 10 -t 60s -H http://api:3000
```