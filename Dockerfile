FROM rust:1.44

WORKDIR /usr/src/aretheygoodatleaguebot
COPY . .

RUN cargo install --path .

CMD cargo run