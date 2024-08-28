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