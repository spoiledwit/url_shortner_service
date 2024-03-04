# Use a Debian base image to match the runtime environment
FROM debian:bullseye as builder

# Install Rust
RUN apt-get update && apt-get install -y curl build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create a new empty shell project
RUN USER=root cargo new --bin url_shortner
WORKDIR /url_shortner

# Copy over your manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY ./src ./src

# Build for release
RUN rm -f ./target/release/deps/url_shortner*
RUN cargo build --release

# Final stage
FROM debian:bullseye
COPY --from=builder /url_shortner/target/release/url_shortner /usr/local/bin/url_shortner
EXPOSE 8080
CMD ["url_shortner"] 