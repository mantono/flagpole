FROM rust:1.76-slim-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin flagpole
WORKDIR /flagpole

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/flagpole*
RUN cargo build --release

# our final base
FROM rust:1.49

# copy the build artifact from the build stage
COPY --from=build /flagpole/target/release/flagpole .

ENV HOST=0.0.0.0
ENV PORT=3000
ENV LOG_LEVEL=INFO
ENV API_KEY=

# set the startup command to run your binary
CMD ["./flagpole"]
