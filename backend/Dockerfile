FROM rust:1.85 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM ubuntu:24.04

WORKDIR /lite-wiki

COPY --from=builder /usr/src/app/target/release/lite-wiki-backend /usr/local/bin/lite-wiki

COPY static ./static
COPY data /lite-wiki/data
COPY entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

VOLUME ["/data/docs", "/data/uploads", "/data"]

EXPOSE 3000
CMD ["/entrypoint.sh"]