fn main() {
    tonic_build::compile_protos("proto/todo.proto").unwrap();
}
