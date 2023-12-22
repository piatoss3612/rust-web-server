FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

# Volume binding in linux occurs permission issues, so create a user matching the host user
RUN addgroup --gid 1000 piatoss && \
    adduser --uid 1000 --gid 1000 --gecos "" --disabled-password piatoss

USER piatoss

CMD ["cargo", "watch", "--why", "-x", "build"]

