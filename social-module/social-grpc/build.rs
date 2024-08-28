use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let services = vec!["helloworld", "social_accounts"];
    
    let mut proto_files: Vec<PathBuf> = Vec::new();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    for service in services {
        let service_path = root.join(format!("proto/api/{}/v1", service));
        proto_files.push(service_path.join(format!("{}.proto", service)));
    }
    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_path = out_dir.join("social-grpc.protoset.bin");


    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(descriptor_path.clone())
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &proto_files,
            &[
                &root.join("proto")
            ],
        ).unwrap();
    Ok(())
}

