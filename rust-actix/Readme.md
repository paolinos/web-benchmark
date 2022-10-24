# Rust Actix
Rust + [Actix](https://actix.rs/) framework


### Dev
```bash
docker container run -it --rm -w /app -v $(pwd):/app --network=dev-network -p 3000:3000 --net-alias api --name rust-dev rust sh


# Create project
cargo new rust-actix
```


