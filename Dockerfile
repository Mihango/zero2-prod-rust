# use the latest Rust stable version as base image
FROM rust:1.60.0

# switch to working directory equivalent to "cd app"
# Docker will create the directory if it does not exist
WORKDIR /app

# copy all files from our working directory to Docker image
COPY . .

# build the binary with release profile
RUN cargo build --release

# When docker run is executed, launch the binary
ENTRYPOINT ["./target/release/app/zero2prod"]
