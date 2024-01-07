# rs-tree

Advanced link tree application with user and group management written in Rust.

## Install

TBA

## Build Docker Development Setup

Compilation might fail, if `.sqlx` or migrations are not up to date.

### Build Image and run development setup:

```sh
docker compose up -d --build
```
API location: `http://localhost/api/<endpoint>`  
[OpenAPI location](http://localhost/apidoc/openapi.yaml): `http://localhost/apidoc/openapi.yaml`[^1]  
[Swagger Editor](http://localhost/apidoc/): `http://localhost/apidoc/`

[^1]: May not be up to date and be be aware of browser caching!

### Start only dependent services and run rs-tree:

```sh
docker compose up -d db session
cargo run
```
API location: `http://localhost:8080/api/<endpoint>`

### .sqlx & migrations

Requires [sqlx-cli](https://github.com/launchbadge/sqlx):
```
cargo install sqlx-cli
```

Update `.sqlx` requires an up-to-date database running:
```sh
cargo sqlx prepare
```

Updates database scheme:
```sh
sqlx migrate run
```

## Environment Variables

🚨 **Change Default Passwords and Keys** 🚨

| Variable      | Default / Example                               | Description           |
| :---          | :---                                            | :---                  |
| `URL`         | `/api`                                          | API Base URL          |
| `DATABASE_URL`| `postgres://rs-tree:rs-tree@rs-tree-db/rs-tree` | Postgres Database     |
| `REDIS_URL`   | `redis://rs-tree-session:6379`                  | Redis Database        |
| `COOKIE_KEY`  | `f000NkKUx[...]DRNLYyC5y8SOk9tics/bxTL+etQ==`   | 64 Byte Key in Base64 |
| `ADMIN`       | `RS-Tr33`                                       | Initial Admin Password|

see [.env](./.env) or [docker-compose.yml](./docker-compose.yml)