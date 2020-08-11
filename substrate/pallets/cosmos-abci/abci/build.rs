fn main() {
    tonic_build::compile_protos("proto/types.proto").unwrap();
}
