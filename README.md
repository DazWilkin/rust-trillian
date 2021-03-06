# Rust SDK for [Google Trillian](github.com/google/trillian)

Intends to provide current (!) rust sources for Trillian's protobufs.

Trillian's protos are themselves dependent on Google APIs and so this crate takes a dependency on [rust-googleapis](ttps://github.com/DazWilkin/rust-googleapis.git)

The crate clones [Trillian](https://github.com/google/trillian) and [Google APIs](https://github.com/googleapis/googleapis.git) which contains Google's public interface definitions (including protos).

The result is a crate called `rust-trillian` that includes a `google` module that includes the various Trillian modules:

+ `trillian`
+ `trillian_admin_api`
+ `trillian_admin_api_grpc`
+ `trillian_log_api`
+ `trillian_log_api_grpc`
+ `trillian_log_sequencer_api`
+ `trillian_log_sequencer_api_grpc`
+ `trillian_map_api`
+ `trillian_map_api_grpc`

and their dependents:

+ `keyspb`
+ `sigpb`

> **NB** The `google` module may be redundant

## Structure

Trillian's protos are split across multiple files to partition by functionality, e.g. Trillian Log, Trillian Map. With Golang, Java and others, it's possible to publish these (different files) into a singular namespace. With Rust, different files denotes different modules and so the Rust module structure includes an additional layer that's not present in the other languages.

```bash
.
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── googleapis
├── protoc-3.11.4-linux-x86_64
├── README.md
├── src
│   ├── crypto
│   │   ├── keyspb.rs
│   │   ├── mod.rs
│   │   └── sigpb.rs

│   ├── trillian
│   │   ├── trillian_admin_api_grpc.rs
│   │   ├── trillian_admin_api.rs
│   │   ├── trillian_log_api_grpc.rs
│   │   ├── trillian_log_api.rs
│   │   ├── trillian_log_sequencer_api_grpc.rs
│   │   ├── trillian_log_sequencer_api.rs
│   │   ├── trillian_map_api_grpc.rs
│   │   ├── trillian_map_api.rs
│   │   └── trillian.rs
│   └── lib.rs
├── target
└── trillian
```

## Test

'clean room' build of the repo to ensure that it builds.

```bash
PROJECT="crate-transparency"
IMAGE="rust-trillian"
TAG=$(git rev-parse HEAD)

docker build \
--tag=gcr.io/${PROJECT}/${IMAGE}:${TAG} \
--file=./Dockerfile.test \
./src
```