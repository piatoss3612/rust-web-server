# Rust Web App

## Database

### Setup Diesel CLI

```bash
$ docker exec -it app diesel setup
```

### Create Migration

```bash
$ docker exec -it app diesel migration generate create_rustaceans
Creating migrations/2023-12-22-011338_create_rustaceans/up.sql
Creating migrations/2023-12-22-011338_create_rustaceans/down.sql
```

```bash
$ docker exec -it app diesel migration generate crates
Creating migrations/2023-12-22-011350_crates/up.sql
Creating migrations/2023-12-22-011350_crates/down.sql
```

### Run Migration

```bash
$ docker exec -it app diesel migration run
Running migration 2023-12-22-020545_create_rustaceans
Running migration 2023-12-22-020550_crates
```

## Run App

```bash
$ docker exec -it app cargo run
```

### Call API

```bash
$ docker exec -it app bash
$ curl 127.0.0.1:8000/rustaceans -H 'Content-Type: application/json' -H 'Accept: application/json' -d '{"name":"John Doe", "email":"john.doe@email.com"}'
$ curl 127.0.0.1:8000/rustaceans
$ curl 127.0.0.1:8000/rustaceans/1
$ curl 127.0.0.1:8000/rustaceans/1 -X PUT -H 'Content-Type: application/json' -H 'Accept: application/json' -d '{"name":"John Doe", "email":"john.doe2@email.com"}'
$ curl -X DELETE 127.0.0.1:8000/rustaceans/1
```