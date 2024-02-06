use std::io::Result;
use protobuf_zmq_rust_generator::ZmqServerGenerator;

fn main() -> Result<()> {
    std::env::set_var("PROTOC", "/home/outerlook/anaconda3/bin/protoc");
    prost_build::Config::new()
        .out_dir("src/generated/")
        .service_generator(Box::new(ZmqServerGenerator {}))
        .compile_protos(&["prover.proto"], &["src/"])?;

    Ok(())
}