# Newsletter backend

![example workflow](https://github.com/nadeemb53/rs-zero2prod/actions/workflows/general.yml/badge.svg)

Health check: [production](https://zero2prod-y3ywz.ondigitalocean.app/health_check)

## Pre-requisites

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

There are also some OS-specific requirements.

### Windows
  
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

```
cargo install --version=0.6.0 sqlx-cli --no-default-features --features postgres
```

### Linux

```bash
# Ubuntu 
sudo apt-get install lld clang libssl-dev postgresql-client
# Arch 
sudo pacman -S lld clang postgresql
```

```
cargo install --version=0.6.0 sqlx-cli --no-default-features --features postgres
```

### MacOS

```bash
brew install michaeleisel/zld/zld
```

```
cargo install --version=0.6.0 sqlx-cli --no-default-features --features postgres
```

## How to build

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

Launch a Redis instance via Docker:

```bash
./scripts/init_redis.sh
```

Launch `cargo`:

```bash
cargo build
```

You can now try with opening a browser on http://127.0.0.1:8000/login after
having launch the web server with `cargo run`.

There is a default `admin` account with password
`everythinghastostartsomewhere`. The available entrypoints are listed in
`src/startup.rs`

## How to test

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

Launch `cargo`:

```bash
cargo test 
```