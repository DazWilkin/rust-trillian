use git2::{ErrorCode, Repository};

use std::path::Path;

pub mod crypto {
    pub fn keyspb() {
        match protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/crypto",
            includes: &["trillian"],
            input: &["trillian/crypto/keyspb/keyspb.proto"],
            rust_protobuf: true,
            ..Default::default()
        }) {
            Ok(_) => println!("Compiled"),
            Err(e) => panic!("{:?}", e),
        };
    }
    pub fn sigpb() {
        match protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/crypto",
            includes: &["trillian"],
            input: &["trillian/crypto/sigpb/sigpb.proto"],
            rust_protobuf: true,
            ..Default::default()
        }) {
            Ok(_) => println!("Compiled"),
            Err(e) => panic!("{:?}", e),
        };
    }
}
pub mod trillian {
    pub fn trillian() {
        match protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/trillian",
            includes: &["trillian", "googleapis"],
            input: &[
                "trillian/trillian_admin_api.proto",
                "trillian/trillian_log_api.proto",
                "trillian/trillian_log_sequencer_api.proto",
                "trillian/trillian_map_api.proto",
                "trillian/trillian.proto",
            ],
            rust_protobuf: true,
            ..Default::default()
        }) {
            Ok(_) => println!("Compiled"),
            Err(e) => panic!("{:?}", e),
        };
    }
}

fn main() {
    let googleapis = "googleapis";
    match Repository::clone(
        "https://github.com/google/googleapis.git",
        Path::new(googleapis),
    ) {
        Ok(_) => println!("[{}] cloned", googleapis),
        Err(e) => match e.code() {
            ErrorCode::Exists => println!("[{}] exists", googleapis),
            _ => panic!(
                "[{}] unexpected: {:?} {:?}",
                googleapis,
                e.code(),
                e.message()
            ),
        },
    };

    let trillian = "trillian";
    match Repository::clone(
        "https://github.com/google/trillian.git",
        Path::new(trillian),
    ) {
        // Clone succeeds, proceed
        // If error is (directory) exists, (probably!) don't need to reclone
        Ok(_) => {
            println!("[{}] cloned", trillian);
            crypto::keyspb();
            crypto::sigpb();
            trillian::trillian();
        }
        Err(e) => match e.code() {
            ErrorCode::Exists => println!("[{}] exists", trillian),
            _ => panic!(
                "[{}] unexpected: {:?} {:?}",
                trillian,
                e.code(),
                e.message()
            ),
        },
    };
}
