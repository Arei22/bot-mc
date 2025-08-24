# Build image
# Necessary dependencies to build scan-website-discord-bot
FROM rust:alpine AS build

LABEL version="0.0.1" maintainer="Asthowen<contact@asthowen.fr>"

RUN apk update && apk upgrade
RUN apk add --no-cache \
    musl-dev alpine-sdk build-base \
    postgresql-dev perl perl-dev openssl-dev libssl3 libcrypto3 openssl-libs-static \
    pkgconfig

WORKDIR "/scan-website-discord-bot"

COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

# Release image
# Necessary dependencies to run scan-website-discord-bot
FROM alpine:latest

RUN apk add --no-cache --update tzdata
ENV TZ=Europe/Paris
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

WORKDIR "/scan-website-discord-bot"

COPY --from=build /scan-website-discord-bot/target/x86_64-unknown-linux-musl/release/scan-website-discord-bot ./scan-website-discord-bot

CMD ["./scan-website-discord-bot"]