# Api
c# & .net core 6


### Dev
Development container to run the app
```bash
docker container run -it --rm -p 3000:3000 --network=dev-network --net-alias api -w /app -v ${PWD}:/app --name net-core-dev mcr.microsoft.com/dotnet/sdk:6.0-alpine3.16
```