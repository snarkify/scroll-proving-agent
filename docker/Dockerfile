# Use the official Rust image as the base image
FROM --platform=linux/amd64 scrolltech/go-rust-builder:go-1.21-rust-nightly-2023-12-03 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the entire project
COPY . .

# Build the project
RUN cargo build --release

# Create a new stage with a minimal Ubuntu image
FROM --platform=linux/amd64 ubuntu:20.04

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/scroll-proving-agent /usr/local/bin/scroll-proving-agent

# Set the entrypoint
ENTRYPOINT ["scroll-proving-agent"]
