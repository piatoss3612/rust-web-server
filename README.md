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
