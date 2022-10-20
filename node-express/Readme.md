# Node - Expressjs
Using [Express](http://expressjs.com/) framework



### Dev
```
# Dev
docker container run -it --rm -w /app -v ${PWD}:/app --network=dev-network -p 3000:3000 --net-alias api --name node-dev node:alpine3.16 sh
```