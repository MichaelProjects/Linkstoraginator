FROM rust:1.77.2-alpine3.19 as builder
COPY . .
RUN apk update && apk add openssl-dev gcc musl-dev build-base
RUN cargo build --release

FROM alpine:3.19.1
RUN apk --no-cache add openssl ca-certificates libressl

WORKDIR /app
COPY --from=builder target/release/recipe_storage /app/recipe_storage
COPY templates /app/templates
COPY public /app/public
RUN adduser -D -u 8877 notroot

RUN mkdir -p /app/logs/
RUN chmod -R 777 /app/logs

USER notroot
ENV db_url "mongodb://localhost:27017"

CMD ["/app/recipe_storage"]
