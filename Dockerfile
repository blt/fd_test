FROM rust:latest

RUN apt-get update && apt-get install lsof

WORKDIR /usr/src/file_descriptor_test
COPY . .

RUN cargo install

CMD ["fd_test"]
