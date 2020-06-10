FROM rust:1.43 as builder
WORKDIR /usr/src/db-changes-server
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libsqlite3-dev
COPY --from=builder /usr/local/cargo/bin/db-changes-server /usr/local/bin/db-changes-server
CMD ["db-changes-server"]
