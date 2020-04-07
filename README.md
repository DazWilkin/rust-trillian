# Rust SDK for [Google Trillian](github.com/google/trillian)

Intends to provide current (!) rust sources for Trillian's protobufs.

Trillian's protos are themselves dependent on Google APIs and so this crate takes a dependency on [rust-googleapis](ttps://github.com/DazWilkin/rust-googleapis.git)

The crate clones [Trillian](https://github.com/google/trillian) and [Google APIs](https://github.com/googleapis/googleapis.git) which contains Google's public interface definitions (including protos).

The result is a crate called `rust-trillian` that includes a `google` module that includes the various Trillian modules:

+ trillian
+ trillian_admin_api
+ trillian_admin_api_grpc
+ trillian_log_api
+ trillian_log_api_grpc
+ trillian_log_sequencer_api
+ trillian_log_sequencer_api_grpc
+ trillian_map_api
+ trillian_map_api_grpc

and their dependents:

+ keyspb
+ sigpb

> **NB** The `google` module may be redundant

## Structure

```bash
.
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── googleapis
│   ├── google
│   │   ├── api
│   │   ├── rpc
│   ├── grafeas
├── protoc-3.11.4-linux-x86_64
├── README.md
├── src
│   ├── google
│   │   ├── keyspb.rs
│   │   ├── mod.rs
│   │   ├── sigpb.rs
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