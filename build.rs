use git2::Repository;

use std::path::Path;

fn main() {
    Repository::clone(
        "https://github.com/googleapis/googleapis.git",
        Path::new("googleapis"),
    )
    .expect("failed to clone");
    Repository::clone(
        "https://github.com/google/trillian.git",
        Path::new("trillian"),
    )
    .expect("failed to clone");

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/google",
        includes: &["googleapis", "trillian"],
        input: &[
            "trillian/trillian_admin_api.proto",
            "trillian/trillian_log_api.proto",
            "trillian/trillian_log_sequencer_api.proto",
            "trillian/trillian_map_api.proto",
            "trillian/trillian.proto",
            "trillian/crypto/keyspb/keyspb.proto",
            "trillian/crypto/sigpb/sigpb.proto",
        ],
        rust_protobuf: true,
        ..Default::default()
    })
    .expect("protoc");
}
