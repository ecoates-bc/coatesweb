FROM rust:latest
WORKDIR /rocket
COPY . .

RUN rustup toolchain install nightly
EXPOSE 8000