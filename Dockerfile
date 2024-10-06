# Use the official Rust image as a build stage
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/discord-role-icon-bot

# Copy the source code into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/myapp/target/release/discord-role-icon-bot .

# Set the entrypoint to the compiled binary
ENTRYPOINT ["./discord-role-icon-bot"]
