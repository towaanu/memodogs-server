FROM rust:1.63-alpine3.15 AS build

COPY Cargo.toml /memodogs/
COPY ./src /memodogs/src

WORKDIR /memodogs

RUN apk update && apk add --no-cache \
    musl-dev

RUN cargo build --release

FROM alpine

RUN mkdir /app

COPY --from=build /memodogs/target/release/memodogs /app/memodogs

COPY ./assets /app/assets
ENV MEMODOGS_IMAGES_PATH=/app/assets/images

CMD ["/app/memodogs"]
