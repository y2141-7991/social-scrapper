mod client;
mod server;
mod api;

fn main() {
    use std::path::{Path, PathBuf};
    let services = vec!["helloworld", "social_accounts"];
    
    let mut proto_files: Vec<PathBuf> = Vec::new();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for service in services {
        let service_path = root.join(format!("proto/api/{}/v1", service));
        proto_files.push(service_path.join(format!("{}.proto", service)));
    }
    println!("{:?}", proto_files);
}