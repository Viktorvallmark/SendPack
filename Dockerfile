FROM ubuntu:latest
FROM rust:latest
FROM kubernetes:latest
LABEL authors="Viktor"

ENTRYPOINT ["top", "-b"]