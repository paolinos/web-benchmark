FROM mcr.microsoft.com/dotnet/sdk:6.0-alpine3.16 AS build-project
WORKDIR /app

COPY ./*.csproj ./
RUN dotnet restore

COPY ./Controllers ./Controllers
COPY ./Program.cs ./Program.cs
RUN dotnet publish -c Release -o out

# Production image
FROM mcr.microsoft.com/dotnet/aspnet:6.0-alpine3.16
WORKDIR /app
COPY --from=build-project /app/out .
EXPOSE 3000
ENTRYPOINT ["dotnet", "netcore.dll"]