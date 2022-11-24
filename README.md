<h1 align="center">Craww</h3>

Gemini Crawler written in Rust. Information Retrieval project of [Arghyadip](https://github.com/arghyadipchak/) and [Gurdit](https://github.com/arghyadipchak/) @[CMI](https://www.cmi.ac.in)

## Getting Started

### For Docker (Recommended)

1. Install docker and docker-compose-plugin
2. Clone the repository
```sh
git clone https://github.com/arghyadipchak/craww
```
3. Create a config.toml file (example config given)
4. Build and Run
```sh
docker compose up
```
### For Non-Docker

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository
```sh
git clone https://github.com/arghyadipchak/craww
```
3. Build Craww
```sh
cargo build --release
```
4. Create a `config.toml` file (example config below)
5. Run Craww
```sh
./target/release/craww
```
OR You can run Craww directly with
```sh
cargo run
```

## Configuration

Example config file (`config.toml`)

```toml
root = "gemini.circumlunar.space" #Root Seed
timeout = 5                       #Connection Timeout(in secs)
database = "store.db"             #Sqlite file

[cache]                           #Bloom Filter config
expected_web_pages = 100000
false_positive_rate = 0.01
```