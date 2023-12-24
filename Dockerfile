FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

# volume binding in linux occurs permission issues, so create a user matching the host user
RUN addgroup --gid 1000 piatoss && \
    adduser --uid 1000 --gid 1000 --gecos "" --disabled-password piatoss

# permissions for the user
RUN chown -R piatoss:piatoss /app

# add the user to sudoers
RUN usermod -aG sudo piatoss

# install sudo
RUN apt-get update && apt-get install -y sudo

# change ownership of the cargo
RUN sudo chown -R piatoss:piatoss /usr/local/cargo

USER piatoss

CMD ["cargo", "watch", "--why", "--", "echo"]