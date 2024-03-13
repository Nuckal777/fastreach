FROM rust:1.76-bookworm as build
WORKDIR /app
COPY . /app
ENV RUSTFLAGS="-C target-cpu=native"
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /app/target/release/fastreach-api /
ENV FASTREACH_GRAPH=/data/graph.bin
EXPOSE 8080
STOPSIGNAL SIGINT
CMD ["/fastreach-api"]
