FROM node:20-alpine as ui-build
COPY . /app
WORKDIR /app/fastreach-ui
RUN npm install && npm run build

FROM rust:1.81-bookworm as build
WORKDIR /app
COPY . /app
ENV RUSTFLAGS="-C target-cpu=native"
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /app/target/release/fastreach-api /
COPY --from=ui-build /app/fastreach-ui/dist /usr/share/fastreach
ENV FASTREACH_GRAPH=/data/graph.bin
ENV FASTREACH_STATIC=/usr/share/fastreach
EXPOSE 8080
STOPSIGNAL SIGINT
CMD ["/fastreach-api"]
