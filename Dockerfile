ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

ADD . ./

RUN sudo chown -R kitanoyoru:kitanoyoru /home/kitanoyoru

RUN cargo build --release


FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-actix-example \
    /usr/local/bin/

CMD /usr/local/bin/rust-actix-example
