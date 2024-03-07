FROM docker.io/library/rust:1.76-alpine3.19 AS build

COPY . /build

WORKDIR /build

RUN apk add build-base pkgconfig musl-dev openssl-dev openssl-libs-static
RUN cargo build --release

FROM docker.io/library/alpine:3.19

ARG PG_VERSION=16

RUN apk add postgresql${PG_VERSION}-client

WORKDIR /app

COPY --from=build /build/target/release/pg-auto-backup /app

VOLUME [ "/app/config"]
VOLUME [ "/var/lib/pg-auto-backup" ]

ENTRYPOINT [ "/app/pg-auto-backup" ]
CMD [ "-c", "/app/config/config.toml" ]
