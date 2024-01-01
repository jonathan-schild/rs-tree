# rs-tree

Advanced link tree application with user and group management written in Rust.

## Environment Variables
:rotating_light: **Change Default Passwords and Keys** :rotating_light:
| Variable      | Default / Example                               | Description           |
| :---          | :---                                            | :---                  |
| `URL`         | `/api`                                          | API Base URL          |
| `DATABASE_URL`| `postgres://rs-tree:rs-tree@rs-tree-db/rs-tree` | Postgres Database     |
| `REDIS_URL`   | `redis://rs-tree-session:6379`                  | Redis Database        |
| `COOKIE_KEY`  | `f000NkKUx[...]DRNLYyC5y8SOk9tics/bxTL+etQ==`   | 64 Byte Key in Base64 |