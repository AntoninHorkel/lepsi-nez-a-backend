FROM nixos/nix
WORKDIR /app
COPY . .
# RUN cargo install --path .
RUN nix run . --extra-experimental-features "nix-command flakes"
# FROM debian:bullseye-slim
# # RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/lepsi-nez-a-backend /usr/local/bin/lepsi-nez-a-backend
# CMD ["lepsi-nez-a-backend"]
