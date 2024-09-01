pub mod social {
    pub mod api {
        pub mod social_account {
            pub mod v1 {
                tonic::include_proto!("social.api.social_accounts.v1");
            }
        }
        pub mod helloworld {
            pub mod v1 {
                tonic::include_proto!("social.api.helloworld.v1");
            }
        }
    }
}

pub mod _reflection {
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("social-grpc.protoset");
}
