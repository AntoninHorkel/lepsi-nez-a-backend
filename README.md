# Lepší než áčko backend

## Environment variables

- DATABASE_URL: PostgreSQL database URL
- ROUTER_URL: the router will listen on this URL, optional, defaults to 127.0.0.1:6767

## Hard-reset database

```sh
docker compose down -v
docker compose up -d
sqlx migrate run
```

## Schéma databáze (deprecated)

![Schéma databáze](assets/schema.jpg)

## Endpointy (deprecated)

![Endpointy](assets/endpoints.jpg)
