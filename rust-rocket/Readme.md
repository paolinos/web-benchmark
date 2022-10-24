# Rust - Rocket
[Rocket](https://rocket.rs/)

**Examples**
Here you can check more [examples](https://github.com/SergioBenitez/Rocket/tree/v0.5-rc/examples)

### Dev
```bash
docker container run -it -w /app -v $(pwd):/app --network=dev-network -p 3000:3000 --net-alias api --name rust-dev rust sh
```

### Run
```bash
# Development run 
cargo run

cargo run --release

# Production settings
ROCKET_ENV=production cargo run
```