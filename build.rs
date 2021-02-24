fn main() {
    prost_build::compile_protos(&["proto/main/v1/main.proto"], &["./proto/"]).unwrap();
}
