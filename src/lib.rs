pub mod tensorflow {
    pub mod serving {
        include!("tensorflow.serving.rs");
    }
    pub mod error {
        include!("tensorflow.error.rs");
    }
    include!("tensorflow.rs");
}
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
