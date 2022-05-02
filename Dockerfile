FROM rust:1.60 as build

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/rustykozle*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /app/target/release/rustykozle .

# set default environment variables
ENV RUST_LOG=info,tracing::span=warn,serenity::http=warn,serenity::gateway=warn,serenity::client::dispatch=warn
ENV RUST_BACKTRACE=1

# set the startup command to run your binary
CMD ["./rustykozle"]
