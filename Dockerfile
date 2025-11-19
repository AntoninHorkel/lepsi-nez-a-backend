FROM nixos/nix
WORKDIR /app
COPY . .
# RUN nix run . --extra-experimental-features "nix-command flakes"
RUN nix develop . --extra-experimental-features "nix-command flakes" -c sh -c "sqlx database setup && cargo r --release"
