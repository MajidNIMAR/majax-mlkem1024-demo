FROM rust:1.98-bookworm@sha256:82150a52ec202c1b14d7817e14516c392bb7f5cfebd88f1ed531cb37ebd39922 AS engine-builder

ARG SOURCE_DATE_EPOCH=0
ENV CARGO_INCREMENTAL=0 \
    SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH} \
    RUSTFLAGS="-C link-arg=-Wl,--build-id=none -C strip=symbols --remap-path-prefix=/src=."

WORKDIR /src
COPY engine/Cargo.toml engine/Cargo.lock ./
COPY engine/src ./src
RUN cargo build --release --locked

FROM scratch AS binary
COPY --from=engine-builder /src/target/release/mlkem-cli /mlkem-cli

FROM node:20-bookworm-slim@sha256:2cf067cfed83d5ea958367df9f966191a942351a2df77d6f0193e162b5febfc0 AS runtime

ENV NODE_ENV=production \
    PORT=8080 \
    MLKEM_CLI_PATH=/opt/majax-kem-demo/bin/mlkem-cli

WORKDIR /opt/majax-kem-demo

COPY --chown=node:node server.mjs ./server.mjs
COPY --chown=node:node public ./public
COPY --chown=node:node scripts/node ./scripts
COPY --chown=node:node LICENSE NOTICE THIRD_PARTY_NOTICES.md ./
COPY --from=engine-builder --chown=node:node /src/target/release/mlkem-cli ./bin/mlkem-cli

RUN chmod 0555 ./bin/mlkem-cli \
    && chmod 0444 ./server.mjs ./public/* ./scripts/* \
    ./LICENSE ./NOTICE ./THIRD_PARTY_NOTICES.md

USER node

EXPOSE 8080

CMD ["node", "server.mjs"]
