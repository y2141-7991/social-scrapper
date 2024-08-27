use std::error::Error;
use std::{env, path::PathBuf, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let services = vec!["helloworld", "social_accounts"];
    
    let mut proto_files: Vec<PathBuf> = Vec::new();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for service in services {
        let service_path = root.join(format!("proto/api/{}/v1", service));
        proto_files.push(service_path.join(format!("{}.proto", service)));
    }
    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let _ = tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .client_mod_attribute(".", r#"#[cfg(feature = "client")]"#)
        .server_mod_attribute(".", r#"#[cfg(feature = "server")]"#)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &proto_files,
            &[
                root
            ],
        );
    Ok(())
}

