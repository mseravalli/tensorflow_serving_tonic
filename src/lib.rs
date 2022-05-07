pub mod tensorflow {
    pub mod error {
        include!("tensorflow.error.rs");
    }
    pub mod serving {
        include!("tensorflow.serving.rs");
    }
    include!("tensorflow.rs");
}
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
