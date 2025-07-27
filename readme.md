```
# Development Instructions

# Build the project
cargo build

# Run the project
cargo run
```

```
# Build Instructions
docker build -t ouka-rs .

# Run the container
# simple run for testing
docker run -p 3000:3000 ouka-rs
# then just simply visit localhost:3000
```

```
# NOTE : please learn about traefik first
docker compose up -d
```
