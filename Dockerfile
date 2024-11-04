FROM rust:latest AS builder

RUN update-ca-certificates

RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser

WORKDIR /emailer

COPY ./ .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /emailer/target/release/emailer /

CMD ["/emailer"]

