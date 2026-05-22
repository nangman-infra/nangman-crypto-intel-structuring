FROM public.ecr.aws/docker/library/rust:1.94-bookworm AS builder

WORKDIR /opt/nangman-crypto/intel-structuring

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY . /opt/nangman-crypto/intel-structuring

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime

COPY --from=builder --chown=nonroot:nonroot \
    /opt/nangman-crypto/intel-structuring/target/release/intel-structuring-app \
    /usr/local/bin/intel-structuring-app

USER nonroot:nonroot

ENV AWS_SDK_LOAD_CONFIG=1

ENTRYPOINT ["/usr/local/bin/intel-structuring-app"]
