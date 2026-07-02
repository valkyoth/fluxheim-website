FROM rust:1.96.1-slim-bookworm AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock* rust-toolchain.toml build.rs ./
COPY src/ src/
COPY content/ content/
COPY config/ config/

RUN cargo build --release --locked

FROM cgr.dev/chainguard/wolfi-base

RUN apk add --no-cache ca-certificates-bundle

RUN addgroup -S fluxheim && \
    adduser -S -G fluxheim fluxheim && \
    mkdir -p /srv/fluxheim-website/assets /srv/fluxheim-website/conf && \
    chown -R fluxheim:fluxheim /srv/fluxheim-website

WORKDIR /srv/fluxheim-website

COPY --from=builder /build/target/release/fluxheim-website /usr/local/bin/fluxheim-website
COPY --chown=fluxheim:fluxheim index.html download.html changelog.html cookies.html privacy.html gdpr.html ./
COPY --chown=fluxheim:fluxheim assets/ assets/
COPY --chown=fluxheim:fluxheim conf/ conf/
COPY --chown=fluxheim:fluxheim docs/ docs/
COPY --chown=fluxheim:fluxheim localized/ localized/

EXPOSE 8080

USER fluxheim

ENV FLUXHEIM_HSTS=enabled
ENV FLUXHEIM_OTLP=disabled
ENV OTEL_EXPORTER_OTLP_HEADERS=""
ENV OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf

CMD ["fluxheim-website"]
