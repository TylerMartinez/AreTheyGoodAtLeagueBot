FROM rust:1.44

WORKDIR /usr/src/aretheygoodatleaguebot
COPY . .

RUN cargo install --path .

CMD ["are_they_good_at_league_bot"]