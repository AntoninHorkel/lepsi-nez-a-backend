FROM nixos/nix
WORKDIR /app
COPY . .
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
# RUN nix run . --extra-experimental-features "nix-command flakes"
RUN nix develop .#slim --extra-experimental-features "nix-command flakes" -c sh -c "sqlx database setup && cargo r --release"
