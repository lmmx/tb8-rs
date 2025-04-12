FROM rust:1.76 as builder

WORKDIR /usr/src/app
COPY . .

# Build with release optimizations
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Install OpenSSL dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/tb8-rs .

# Create .env file for TfL API credentials
RUN touch .env

# Set the PORT environment variable
ENV PORT=4000

# Expose the port
EXPOSE 4000

# Set the command to run the binary
CMD ["./tb8-rs"]