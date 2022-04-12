FROM rust:latest
WORKDIR /usr/src/myapp
COPY  . .
RUN cargo install --path .
CMD  ["speedtestserver",":::2233","10"]
