FROM ubuntu:latest
FROM rust:latest
LABEL authors="Viktor"

ENTRYPOINT ["top", "-b"]