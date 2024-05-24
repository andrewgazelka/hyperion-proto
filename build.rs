fn main() {
    let protos = ["src/proxy_to_server.proto", "src/server_to_proxy.proto"];

    prost_build::Config::new()
        .bytes(["PlayerPackets.data"])
        .compile_protos(&protos, &["src/"])
        .expect("Failed to compile Protobuf files");
}
