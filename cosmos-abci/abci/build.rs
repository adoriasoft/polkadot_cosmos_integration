fn main() {
    tonic_build::configure()
        .type_attribute(".google.protobuf.Timestamp", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/types.proto"], &["proto"])
        .unwrap();
}
