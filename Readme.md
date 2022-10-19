# Web benchmark

I'm trying to do a simple test, just a run some benchmark in different frameworks/langauges and all be running in docker.  
So to run the benchmark I'm going to use [Locust](https://locust.io/) a python load testing tool.

The projects will have the same endpoints, you can take a look to the [open-api](./open-api.spec.yaml) specification

### Folders
- node-fastify => Fastify nodejs project [Readme](./node-fastify/Readme.md)
- locust => Locust load test tool [Readme](./locust/Readme.md)


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

**Locust:**
for more info about locust settings please take a look to the [Readme](./locust/Readme.md)

```bash
# Remember docker volume
#   $(pwd) => Linux/Unix
#   ${PWD} => Powershell - Windows
docker container run --rm -p 8089:8089 --network=dev-network -w /locust -v ${PWD}/locust:/locust locustio/locust -f api-benchmark.py --headless --users 100 --spawn-rate 10 -t 60s -H http://api:3000
```