FROM rust:1.48.0
LABEL maintainer="Matthew Brown <matt.brown@ederoyd.co.uk>"

RUN apt-get update && apt-get install -y \
     musl musl-dev musl-tools

RUN rustup component add rust-std-x86_64-unknown-linux-musl
