FROM ubuntu:focal

RUN apt-get update
RUN apt-get install -y openssl libssl-dev ca-certificates curl
RUN update-ca-certificates


ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY target/release/cron ./

USER app:app

EXPOSE 3000

CMD ["/app/cron"]
