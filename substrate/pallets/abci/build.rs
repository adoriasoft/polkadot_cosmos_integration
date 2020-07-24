fn main() {
    tonic_build::compile_protos("proto/abci.proto").unwrap();
    tonic_build::compile_protos("proto/types.proto").unwrap();
}
