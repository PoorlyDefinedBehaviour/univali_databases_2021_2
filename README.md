## [Install Rust](https://www.rust-lang.org/tools/install)

## Install openssl if you don't have it already

```console
sudo apt install pkg-config libssl-dev
```

## Install sqlx CLI

```console
cargo install sqlx-cli --locked --no-default-features --features mysql
```

## Setup local environment

I will use Docker, if you have MySQL on your system, that will work too.

I you choose to use the MySQL thats installed on your system,
make sure its credentials match the credentials in the `.env` file.

```console
docker-compose -f docker-compose-loc.yml up -d
```

## Run migrations

```console
// Creates sol_nascente database
sqlx database create

// Runs migrations
sqlx migrate run
```

## Running the app

```console
RUST_LOG=databases_1_m3_1=trace HOST=127.0.0.1 PORT=8080 cargo run
```
