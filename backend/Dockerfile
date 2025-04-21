FROM rust:1.85

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

WORKDIR /lite-wiki
RUN cp /usr/src/app/target/release/lite-wiki-backend /usr/local/bin/lite-wiki

COPY data ./data

EXPOSE 3000
CMD ["lite-wiki"]