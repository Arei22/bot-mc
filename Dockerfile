# Build image
# Necessary dependencies to build bot-mc
FROM rust:alpine AS build

LABEL version="0.2.3" maintainer="Arei2<contact@arei2.fr>"

RUN apk update && apk upgrade
RUN apk add --no-cache \
    musl-dev alpine-sdk build-base \
    postgresql-dev perl perl-dev openssl-dev libssl3 libcrypto3 openssl-libs-static \
    pkgconfig

WORKDIR "/bot-mc"

COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

# Release image
# Necessary dependencies to run bot-mc
FROM alpine:latest

RUN apk add --no-cache --update tzdata && apk add docker-cli && apk add docker-cli-compose
ENV TZ=Europe/Paris
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

WORKDIR "/bot-mc"

COPY --from=build /bot-mc/target/x86_64-unknown-linux-musl/release/bot-mc ./bot-mc

CMD ["./bot-mc"]