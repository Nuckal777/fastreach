FROM rust:1.73-bookworm as build
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /app/target/release/fastreach-api /
ENV FASTREACH_GRAPH=/data/graph.bin
EXPOSE 8080
STOPSIGNAL SIGINT
CMD ["./fastreach-api"]
