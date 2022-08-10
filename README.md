# Rust Taxids API - A simple but efficient server for GenBank Taxids

To start a server using a Docker container engine see three simple steps:

1. Build a rust binary:

```bash
cargo build --release
```

2. Build a docker image:

```bash
docker build . -t biotax
```

3. Fire the API:

```bash
docker run --rm -p 8080:8080 biotax
```
