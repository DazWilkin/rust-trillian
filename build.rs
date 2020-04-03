fn main() {
    // stepancheg/rust-protobuf -- recommended
    // protoc_rust::run(protoc_rust::Args {
    //     out_dir: "src/protos",
    //     includes: &[
    //         "/home/dazwilkin/Projects/trillian",
    //         "/home/dazwilkin/go/pkg/mod/github.com/grpc-ecosystem/grpc-gateway@v1.9.4/third_party/googleapis",
    //     ],
    //     input: &["/home/dazwilkin/Projects/trillian/trillian_log_api.proto",],
    //     customize: protoc_rust::Customize {
    //         ..Default::default()
    //     },
    // })
    // .expect("protoc");

    // protoc_rust::run(protoc_rust::Args {
    //     out_dir: "src/protos",
    //     includes: &["googleapis"],
    //     input: &["protos/greeter.proto"],
    //     customize: protoc_rust::Customize {
    //         ..Default::default()
    //     },
    // })
    // .expect("protoc");

    // stepancheg/rust-protobuf -- alpha
    // protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
    //     out_dir: "./src/protos",
    //     includes: &[
    //         "/home/dazwilkin/Projects/rust-trillian/trillian/",
    //         "/home/dazwilkin/Projects/rust-trillian/grpc-gateway/third_party/googleapis/",
    //     ],
    //     input: &["trillian_log_api.proto"],
    //     customize: protobuf_codegen_pure::Customize {
    //         ..Default::default()
    //     },
    // })
    // .expect("protoc");

    // protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
    //     out_dir: "./src/protos",
    //     includes: &["protos"],
    //     input: &["greeter.proto"],
    //     customize: protobuf_codegen_pure::Customize {
    //         ..Default::default()
    //     },
    // })
    // .expect("protoc");

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/protos",
        includes: &[
            "/home/dazwilkin/Projects/trillian",
            "/home/dazwilkin/go/pkg/mod/github.com/grpc-ecosystem/grpc-gateway@v1.9.4/third_party/googleapis",
        ],
        input: &[
            "/home/dazwilkin/Projects/trillian/trillian_admin_api.proto",
            "/home/dazwilkin/Projects/trillian/trillian_log_api.proto",
            "/home/dazwilkin/Projects/trillian/trillian_log_sequencer_api.proto",
            "/home/dazwilkin/Projects/trillian/trillian_map_api.proto",
            "/home/dazwilkin/Projects/trillian/trillian.proto",
            ],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    })
    .expect("protoc");

    // protoc_rust_grpc::run(protoc_rust_grpc::Args {
    //     out_dir: "src/protos",
    //     includes: &["protos"],
    //     input: &["greeter.proto"],
    //     rust_protobuf: true, // also generate protobuf messages, not just services
    //     ..Default::default()
    // })
    // .expect("protoc");
}
