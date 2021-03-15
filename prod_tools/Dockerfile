FROM rust:alpine AS build

COPY Cargo.toml /memodogs/
COPY ./src /memodogs/src

WORKDIR /memodogs

RUN apk update && apk add --no-cache \
    musl-dev

RUN cargo build --release

FROM alpine

COPY --from=build /memodogs/target/release/memodogs /app/memodogs

CMD ["/app/memodogs"]
