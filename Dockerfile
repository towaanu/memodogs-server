FROM rust:alpine

# cargo fmt
RUN rustup component add rustfmt

RUN apk update && apk add --no-cache \
    musl-dev

CMD ["cargo", "run"]
