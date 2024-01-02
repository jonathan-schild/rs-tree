# rs-tree

Advanced link tree application with user and group management written in Rust.

## Docker Dev

```sh
docker compose up -d --build
```

- `http://localhost/api/<endpoint>`: Application API
- <http://localhost/apidoc/>: [Swagger API Doc](https://swagger.io/tools/swagger-ui/)
- <http://localhost/apidoc/openapi.yaml>: [OpenAPI 3.0 Doc](https://swagger.io/specification/v3/)
- <http://localhost/apidevel/>: [Swagger Editor](https://swagger.io/tools/swagger-editor/)

## Environment Variables
:rotating_light: **Change Default Passwords and Keys** :rotating_light:
| Variable      | Default / Example                               | Description           |
| :---          | :---                                            | :---                  |
| `URL`         | `/api`                                          | API Base URL          |
| `DATABASE_URL`| `postgres://rs-tree:rs-tree@rs-tree-db/rs-tree` | Postgres Database     |
| `REDIS_URL`   | `redis://rs-tree-session:6379`                  | Redis Database        |
| `COOKIE_KEY`  | `f000NkKUx[...]DRNLYyC5y8SOk9tics/bxTL+etQ==`   | 64 Byte Key in Base64 |
| `ADMIN`       | `RS-Tr33`                                       | Initial Admin Password|

## Important Things

- special user `admin` with uuid: `00000000-0000-0000-0000-000000000000` created if number of users is 0.