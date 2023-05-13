# Define the name of the builder stage
FROM rust:1.69 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin tidder
WORKDIR /tidder

# Create mount points for /database and /public
RUN mkdir /database
RUN mkdir /public

# Copy our manifests into the project directory
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build our dependencies, this will be cached because we didn't
# copy our source code yet
RUN cargo build --release
RUN rm src/*.rs

# Now, copy our source code
COPY ./src ./src

# Build our application
RUN rm ./target/release/deps/tidder*
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `tidder`.
FROM debian:buster-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libssl-dev \
        tzdata \
        libsqlite3-dev \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /tidder/target/release/tidder /usr/local/bin/tidder
COPY ./.env /.env

VOLUME ["/database", "/public"]
EXPOSE 3000

# Set the startup command to run our binary
CMD ["tidder"]