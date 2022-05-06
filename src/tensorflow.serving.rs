/// Specifies one or more fully independent input Examples.
/// See examples at:
///     <https://github.com/tensorflow/tensorflow/blob/master/tensorflow/core/example/example.proto>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExampleList {
    #[prost(message, repeated, tag="1")]
    pub examples: ::prost::alloc::vec::Vec<super::Example>,
}
/// Specifies one or more independent input Examples, with a common context
/// Example.
///
/// The common use case for context is to cleanly and optimally specify some
/// features that are common across multiple examples.
///
/// See example below with a search query as the context and multiple restaurants
/// to perform some inference on.
///
/// context: {
///   features: {
///     feature: {
///       key  : "query"
///       value: {
///         bytes_list: {
///           value: [ "pizza" ]
///         }
///       }
///     }
///   }
/// }
/// examples: {
///   features: {
///     feature: {
///       key  : "cuisine"
///       value: {
///         bytes_list: {
///           value: [ "Pizzeria" ]
///         }
///       }
///     }
///   }
/// }
/// examples: {
///   features: {
///     feature: {
///       key  : "cuisine"
///       value: {
///         bytes_list: {
///           value: [ "Taqueria" ]
///         }
///       }
///     }
///   }
/// }
///
/// Implementations of ExampleListWithContext merge the context Example into each
/// of the Examples. Note that feature keys must not be duplicated between the
/// Examples and context Example, or the behavior is undefined.
///
/// See also:
///     tensorflow/core/example/example.proto
///     <https://developers.google.com/protocol-buffers/docs/proto3#maps>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExampleListWithContext {
    #[prost(message, repeated, tag="1")]
    pub examples: ::prost::alloc::vec::Vec<super::Example>,
    #[prost(message, optional, tag="2")]
    pub context: ::core::option::Option<super::Example>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    #[prost(oneof="input::Kind", tags="1, 2")]
    pub kind: ::core::option::Option<input::Kind>,
}
/// Nested message and enum types in `Input`.
pub mod input {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag="1")]
        ExampleList(super::ExampleList),
        #[prost(message, tag="2")]
        ExampleListWithContext(super::ExampleListWithContext),
    }
}
/// Metadata for an inference request such as the model name and version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelSpec {
    /// Required servable name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A named signature to evaluate. If unspecified, the default signature will
    /// be used.
    #[prost(string, tag="3")]
    pub signature_name: ::prost::alloc::string::String,
    /// Optional choice of which version of the model to use.
    ///
    /// Recommended to be left unset in the common case. Should be specified only
    /// when there is a strong version consistency requirement.
    ///
    /// When left unspecified, the system will serve the best available version.
    /// This is typically the latest version, though during version transitions,
    /// notably when serving on a fleet of instances, may be either the previous or
    /// new version.
    #[prost(oneof="model_spec::VersionChoice", tags="2, 4")]
    pub version_choice: ::core::option::Option<model_spec::VersionChoice>,
}
/// Nested message and enum types in `ModelSpec`.
pub mod model_spec {
    /// Optional choice of which version of the model to use.
    ///
    /// Recommended to be left unset in the common case. Should be specified only
    /// when there is a strong version consistency requirement.
    ///
    /// When left unspecified, the system will serve the best available version.
    /// This is typically the latest version, though during version transitions,
    /// notably when serving on a fleet of instances, may be either the previous or
    /// new version.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VersionChoice {
        /// Use this specific version number.
        #[prost(message, tag="2")]
        Version(i64),
        /// Use the version associated with the given label.
        #[prost(string, tag="4")]
        VersionLabel(::prost::alloc::string::String),
    }
}
/// A single class.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Class {
    /// Label or name of the class.
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    /// Score for this class (e.g., the probability the item belongs to this
    /// class). As per the proto3 default-value semantics, if the score is missing,
    /// it should be treated as 0.
    #[prost(float, tag="2")]
    pub score: f32,
}
/// List of classes for a single item (tensorflow.Example).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Classifications {
    #[prost(message, repeated, tag="1")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
}
/// Contains one result per input example, in the same order as the input in
/// ClassificationRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationResult {
    #[prost(message, repeated, tag="1")]
    pub classifications: ::prost::alloc::vec::Vec<Classifications>,
}
// RPC Interfaces

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Input data.
    #[prost(message, optional, tag="2")]
    pub input: ::core::option::Option<Input>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationResponse {
    /// Effective Model Specification used for classification.
    #[prost(message, optional, tag="2")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Result of the classification.
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<ClassificationResult>,
}
/// Message returned for "signature_def" field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureDefMap {
    #[prost(map="string, message", tag="1")]
    pub signature_def: ::std::collections::HashMap<::prost::alloc::string::String, super::SignatureDef>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelMetadataRequest {
    /// Model Specification indicating which model we are querying for metadata.
    /// If version is not specified, will use the latest (numerical) version.
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Metadata fields to get. Currently supported: "signature_def".
    #[prost(string, repeated, tag="2")]
    pub metadata_field: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelMetadataResponse {
    /// Model Specification indicating which model this metadata belongs to.
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Map of metadata field name to metadata field. The options for metadata
    /// field name are listed in GetModelMetadataRequest. Currently supported:
    /// "signature_def".
    #[prost(map="string, message", tag="2")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
}
/// Status that corresponds to Status in
/// third_party/tensorflow/core/lib/core/status.h.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusProto {
    /// Error code.
    #[prost(enumeration="super::error::Code", tag="1")]
    pub error_code: i32,
    /// Error message. Will only be set if an error was encountered.
    #[prost(string, tag="2")]
    pub error_message: ::prost::alloc::string::String,
}
/// GetModelStatusRequest contains a ModelSpec indicating the model for which
/// to get status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelStatusRequest {
    /// Model Specification. If version is not specified, information about all
    /// versions of the model will be returned. If a version is specified, the
    /// status of only that version will be returned.
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
}
/// Version number, state, and status for a single version of a model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelVersionStatus {
    /// Model version.
    #[prost(int64, tag="1")]
    pub version: i64,
    /// Model state.
    #[prost(enumeration="model_version_status::State", tag="2")]
    pub state: i32,
    /// Model status.
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<StatusProto>,
}
/// Nested message and enum types in `ModelVersionStatus`.
pub mod model_version_status {
    /// States that map to ManagerState enum in
    /// tensorflow_serving/core/servable_state.h
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Default value.
        Unknown = 0,
        /// The manager is tracking this servable, but has not initiated any action
        /// pertaining to it.
        Start = 10,
        /// The manager has decided to load this servable. In particular, checks
        /// around resource availability and other aspects have passed, and the
        /// manager is about to invoke the loader's Load() method.
        Loading = 20,
        /// The manager has successfully loaded this servable and made it available
        /// for serving (i.e. GetServableHandle(id) will succeed). To avoid races,
        /// this state is not reported until *after* the servable is made
        /// available.
        Available = 30,
        /// The manager has decided to make this servable unavailable, and unload
        /// it. To avoid races, this state is reported *before* the servable is
        /// made unavailable.
        Unloading = 40,
        /// This servable has reached the end of its journey in the manager. Either
        /// it loaded and ultimately unloaded successfully, or it hit an error at
        /// some point in its lifecycle.
        End = 50,
    }
}
/// Response for ModelStatusRequest on successful run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelStatusResponse {
    /// Version number and status information for applicable model version(s).
    #[prost(message, repeated, tag="1")]
    pub model_version_status: ::prost::alloc::vec::Vec<ModelVersionStatus>,
}
/// Regression result for a single item (tensorflow.Example).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Regression {
    #[prost(float, tag="1")]
    pub value: f32,
}
/// Contains one result per input example, in the same order as the input in
/// RegressionRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressionResult {
    #[prost(message, repeated, tag="1")]
    pub regressions: ::prost::alloc::vec::Vec<Regression>,
}
// RPC interfaces.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressionRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Input data.
    #[prost(message, optional, tag="2")]
    pub input: ::core::option::Option<Input>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressionResponse {
    /// Effective Model Specification used for regression.
    #[prost(message, optional, tag="2")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<RegressionResult>,
}
/// Inference request such as classification, regression, etc...
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferenceTask {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    /// All ModelSpecs in a MultiInferenceRequest must access the same model name.
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Signature's method_name. Should be one of the method names defined in
    /// third_party/tensorflow/python/saved_model/signature_constants.py.
    /// e.g. "tensorflow/serving/classify".
    #[prost(string, tag="2")]
    pub method_name: ::prost::alloc::string::String,
}
/// Inference result, matches the type of request or is an error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferenceResult {
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    #[prost(oneof="inference_result::Result", tags="2, 3")]
    pub result: ::core::option::Option<inference_result::Result>,
}
/// Nested message and enum types in `InferenceResult`.
pub mod inference_result {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag="2")]
        ClassificationResult(super::ClassificationResult),
        #[prost(message, tag="3")]
        RegressionResult(super::RegressionResult),
    }
}
/// Inference request containing one or more requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiInferenceRequest {
    /// Inference tasks.
    #[prost(message, repeated, tag="1")]
    pub tasks: ::prost::alloc::vec::Vec<InferenceTask>,
    /// Input data.
    #[prost(message, optional, tag="2")]
    pub input: ::core::option::Option<Input>,
}
/// Inference request containing one or more responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiInferenceResponse {
    /// List of results; one for each InferenceTask in the request, returned in the
    /// same order as the request.
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<InferenceResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogCollectorConfig {
    /// Identifies the type of the LogCollector we will use to collect these logs.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// The prefix to use for the filenames of the logs.
    #[prost(string, tag="2")]
    pub filename_prefix: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamplingConfig {
    /// Requests will be logged uniformly at random with this probability. Valid
    /// range: [0, 1.0].
    #[prost(double, tag="1")]
    pub sampling_rate: f64,
}
/// Configuration for logging query/responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingConfig {
    #[prost(message, optional, tag="1")]
    pub log_collector_config: ::core::option::Option<LogCollectorConfig>,
    #[prost(message, optional, tag="2")]
    pub sampling_config: ::core::option::Option<SamplingConfig>,
}
/// Metadata logged along with the request logs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogMetadata {
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    #[prost(message, optional, tag="2")]
    pub sampling_config: ::core::option::Option<SamplingConfig>,
    /// List of tags used to load the relevant MetaGraphDef from SavedModel.
    ///
    /// TODO(b/33279154): Add more metadata as mentioned in the bug.
    #[prost(string, repeated, tag="3")]
    pub saved_model_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Config proto for FileSystemStoragePathSource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemStoragePathSourceConfig {
    /// The servables to monitor for new versions, and aspire.
    #[prost(message, repeated, tag="5")]
    pub servables: ::prost::alloc::vec::Vec<file_system_storage_path_source_config::ServableToMonitor>,
    /// How long to wait between file-system polling to look for children of
    /// 'base_path', in seconds.
    ///
    /// If set to zero, filesystem will be polled exactly once. If set to a
    /// negative value (for testing use only), polling will be entirely disabled.
    #[prost(int64, tag="3")]
    pub file_system_poll_wait_seconds: i64,
    /// If true, then FileSystemStoragePathSource::Create() and ::UpdateConfig()
    /// fail if, for any configured servables, the file system doesn't currently
    /// contain at least one version under the base path.
    /// (Otherwise, it will emit a warning and keep pinging the file system to
    /// check for a version to appear later.)
    /// DEPRECATED: Use 'servable_versions_always_present' instead, which includes
    /// this behavior.
    /// TODO(b/30898016): Remove 2019-10-31 or later.
    #[deprecated]
    #[prost(bool, tag="4")]
    pub fail_if_zero_versions_at_startup: bool,
    /// If true, the servable is always expected to exist on the underlying
    /// filesystem. FileSystemStoragePathSource::Create() and ::UpdateConfig() will
    /// fail if, for any configured servables, the file system doesn't currently
    /// contain at least one version under the base path. In addition, if a polling
    /// loop find the base path empty, it will not unload existing servables.
    #[prost(bool, tag="6")]
    pub servable_versions_always_present: bool,
}
/// Nested message and enum types in `FileSystemStoragePathSourceConfig`.
pub mod file_system_storage_path_source_config {
    /// A policy that dictates which version(s) of a servable should be served.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServableVersionPolicy {
        #[prost(oneof="servable_version_policy::PolicyChoice", tags="100, 101, 102")]
        pub policy_choice: ::core::option::Option<servable_version_policy::PolicyChoice>,
    }
    /// Nested message and enum types in `ServableVersionPolicy`.
    pub mod servable_version_policy {
        /// Serve the latest versions (i.e. the ones with the highest version
        /// numbers), among those found on disk.
        ///
        /// This is the default policy, with the default number of versions as 1.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Latest {
            /// Number of latest versions to serve. (The default is 1.)
            #[prost(uint32, tag="1")]
            pub num_versions: u32,
        }
        /// Serve all versions found on disk.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct All {
        }
        /// Serve a specific version (or set of versions).
        ///
        /// This policy is useful for rolling back to a specific version, or for
        /// canarying a specific version while still serving a separate stable
        /// version.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Specific {
            /// The version numbers to serve.
            #[prost(int64, repeated, tag="1")]
            pub versions: ::prost::alloc::vec::Vec<i64>,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PolicyChoice {
            #[prost(message, tag="100")]
            Latest(Latest),
            #[prost(message, tag="101")]
            All(All),
            #[prost(message, tag="102")]
            Specific(Specific),
        }
    }
    /// A servable name and base path to look for versions of the servable.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServableToMonitor {
        /// The servable name to supply in aspired-versions callback calls. Child
        /// paths of 'base_path' are considered to be versions of this servable.
        #[prost(string, tag="1")]
        pub servable_name: ::prost::alloc::string::String,
        /// The path to monitor, i.e. look for child paths of the form base_path/123.
        #[prost(string, tag="2")]
        pub base_path: ::prost::alloc::string::String,
        /// The policy to determines the number of versions of the servable to be
        /// served at the same time.
        #[prost(message, optional, tag="4")]
        pub servable_version_policy: ::core::option::Option<ServableVersionPolicy>,
    }
}
/// Common configuration for loading a model being served.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelConfig {
    /// Name of the model.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Base path to the model, excluding the version directory.
    /// E.g> for a model at /foo/bar/my_model/123, where 123 is the version, the
    /// base path is /foo/bar/my_model.
    ///
    /// (This can be changed once a model is in serving, *if* the underlying data
    /// remains the same. Otherwise there are no guarantees about whether the old
    /// or new data will be used for model versions currently loaded.)
    #[prost(string, tag="2")]
    pub base_path: ::prost::alloc::string::String,
    /// Type of model.
    /// TODO(b/31336131): DEPRECATED. Please use 'model_platform' instead.
    #[deprecated]
    #[prost(enumeration="ModelType", tag="3")]
    pub model_type: i32,
    /// Type of model (e.g. "tensorflow").
    ///
    /// (This cannot be changed once a model is in serving.)
    #[prost(string, tag="4")]
    pub model_platform: ::prost::alloc::string::String,
    /// Version policy for the model indicating which version(s) of the model to
    /// load and make available for serving simultaneously.
    /// The default option is to serve only the latest version of the model.
    ///
    /// (This can be changed once a model is in serving.)
    #[prost(message, optional, tag="7")]
    pub model_version_policy: ::core::option::Option<file_system_storage_path_source_config::ServableVersionPolicy>,
    /// String labels to associate with versions of the model, allowing inference
    /// queries to refer to versions by label instead of number. Multiple labels
    /// can map to the same version, but not vice-versa.
    ///
    /// An envisioned use-case for these labels is canarying tentative versions.
    /// For example, one can assign labels "stable" and "canary" to two specific
    /// versions. Perhaps initially "stable" is assigned to version 0 and "canary"
    /// to version 1. Once version 1 passes canary, one can shift the "stable"
    /// label to refer to version 1 (at that point both labels map to the same
    /// version -- version 1 -- which is fine). Later once version 2 is ready to
    /// canary one can move the "canary" label to version 2. And so on.
    #[prost(map="string, int64", tag="8")]
    pub version_labels: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// Configures logging requests and responses, to the model.
    ///
    /// (This can be changed once a model is in serving.)
    #[prost(message, optional, tag="6")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
}
/// Static list of models to be loaded for serving.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelConfigList {
    #[prost(message, repeated, tag="1")]
    pub config: ::prost::alloc::vec::Vec<ModelConfig>,
}
/// ModelServer config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelServerConfig {
    /// ModelServer takes either a static file-based model config list or an Any
    /// proto representing custom model config that is fetched dynamically at
    /// runtime (through network RPC, custom service, etc.).
    #[prost(oneof="model_server_config::Config", tags="1, 2")]
    pub config: ::core::option::Option<model_server_config::Config>,
}
/// Nested message and enum types in `ModelServerConfig`.
pub mod model_server_config {
    /// ModelServer takes either a static file-based model config list or an Any
    /// proto representing custom model config that is fetched dynamically at
    /// runtime (through network RPC, custom service, etc.).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag="1")]
        ModelConfigList(super::ModelConfigList),
        #[prost(message, tag="2")]
        CustomModelConfig(::prost_types::Any),
    }
}
/// The type of model.
/// TODO(b/31336131): DEPRECATED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModelType {
    Unspecified = 0,
    Tensorflow = 1,
    Other = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadConfigRequest {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<ModelServerConfig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadConfigResponse {
    #[prost(message, optional, tag="1")]
    pub status: ::core::option::Option<StatusProto>,
}
/// Generated client implementations.
pub mod model_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// ModelService provides methods to query and update the state of the server,
    /// e.g. which models/versions are being served.
    #[derive(Debug, Clone)]
    pub struct ModelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ModelServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ModelServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ModelServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ModelServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Gets status of model. If the ModelSpec in the request does not specify
        /// version, information about all versions of the model will be returned. If
        /// the ModelSpec in the request does specify a version, the status of only
        /// that version will be returned.
        pub async fn get_model_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelStatusRequest>,
        ) -> Result<tonic::Response<super::GetModelStatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.ModelService/GetModelStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Reloads the set of served models. The new config supersedes the old one,
        /// so if a model is omitted from the new config it will be unloaded and no
        /// longer served.
        pub async fn handle_reload_config_request(
            &mut self,
            request: impl tonic::IntoRequest<super::ReloadConfigRequest>,
        ) -> Result<tonic::Response<super::ReloadConfigResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.ModelService/HandleReloadConfigRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// PredictRequest specifies which TensorFlow model to run, as well as
/// how inputs are mapped to tensors and how outputs are filtered before
/// returning to user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Input tensors.
    /// Names of input tensor are alias names. The mapping from aliases to real
    /// input tensor names is stored in the SavedModel export as a prediction
    /// SignatureDef under the 'inputs' field.
    #[prost(map="string, message", tag="2")]
    pub inputs: ::std::collections::HashMap<::prost::alloc::string::String, super::TensorProto>,
    /// Output filter.
    /// Names specified are alias names. The mapping from aliases to real output
    /// tensor names is stored in the SavedModel export as a prediction
    /// SignatureDef under the 'outputs' field.
    /// Only tensors specified here will be run/fetched and returned, with the
    /// exception that when none is specified, all tensors specified in the
    /// named signature will be run/fetched and returned.
    #[prost(string, repeated, tag="3")]
    pub output_filter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response for PredictRequest on successful run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictResponse {
    /// Effective Model Specification used to process PredictRequest.
    #[prost(message, optional, tag="2")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Output tensors.
    #[prost(map="string, message", tag="1")]
    pub outputs: ::std::collections::HashMap<::prost::alloc::string::String, super::TensorProto>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionRunRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag="1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Tensors to be fed in the step. Each feed is a named tensor.
    #[prost(message, repeated, tag="2")]
    pub feed: ::prost::alloc::vec::Vec<super::NamedTensorProto>,
    /// Fetches. A list of tensor names. The caller expects a tensor to
    /// be returned for each fetch\[i\] (see RunResponse.tensor). The
    /// order of specified fetches does not change the execution order.
    #[prost(string, repeated, tag="3")]
    pub fetch: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Target Nodes. A list of node names. The named nodes will be run
    /// to but their outputs will not be fetched.
    #[prost(string, repeated, tag="4")]
    pub target: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If true, treat names in feed/fetch/target as alias names than actual tensor
    /// names (that appear in the TF graph). Alias names are resolved to actual
    /// names using `SignatureDef` in SavedModel associated with the model.
    #[prost(bool, tag="6")]
    pub tensor_name_is_alias: bool,
    /// Options for the run call. **Currently ignored.**
    #[prost(message, optional, tag="5")]
    pub options: ::core::option::Option<super::RunOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionRunResponse {
    /// Effective Model Specification used for session run.
    #[prost(message, optional, tag="3")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// NOTE: The order of the returned tensors may or may not match
    /// the fetch order specified in RunRequest.
    #[prost(message, repeated, tag="1")]
    pub tensor: ::prost::alloc::vec::Vec<super::NamedTensorProto>,
    /// Returned metadata if requested in the options.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::RunMetadata>,
}
/// Generated client implementations.
pub mod session_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// SessionService defines a service with which a client can interact to execute
    /// Tensorflow model inference. The SessionService::SessionRun method is similar
    /// to MasterService::RunStep of Tensorflow, except that all sessions are ready
    /// to run, and you request a specific model/session with ModelSpec.
    #[derive(Debug, Clone)]
    pub struct SessionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SessionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SessionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SessionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SessionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Runs inference of a given model.
        pub async fn session_run(
            &mut self,
            request: impl tonic::IntoRequest<super::SessionRunRequest>,
        ) -> Result<tonic::Response<super::SessionRunResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.SessionService/SessionRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassifyLog {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<ClassificationRequest>,
    #[prost(message, optional, tag="2")]
    pub response: ::core::option::Option<ClassificationResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressLog {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<RegressionRequest>,
    #[prost(message, optional, tag="2")]
    pub response: ::core::option::Option<RegressionResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictLog {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<PredictRequest>,
    #[prost(message, optional, tag="2")]
    pub response: ::core::option::Option<PredictResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiInferenceLog {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<MultiInferenceRequest>,
    #[prost(message, optional, tag="2")]
    pub response: ::core::option::Option<MultiInferenceResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionRunLog {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<SessionRunRequest>,
    #[prost(message, optional, tag="2")]
    pub response: ::core::option::Option<SessionRunResponse>,
}
/// Logged model inference request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictionLog {
    #[prost(message, optional, tag="1")]
    pub log_metadata: ::core::option::Option<LogMetadata>,
    #[prost(oneof="prediction_log::LogType", tags="2, 3, 6, 4, 5")]
    pub log_type: ::core::option::Option<prediction_log::LogType>,
}
/// Nested message and enum types in `PredictionLog`.
pub mod prediction_log {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LogType {
        #[prost(message, tag="2")]
        ClassifyLog(super::ClassifyLog),
        #[prost(message, tag="3")]
        RegressLog(super::RegressLog),
        #[prost(message, tag="6")]
        PredictLog(super::PredictLog),
        #[prost(message, tag="4")]
        MultiInferenceLog(super::MultiInferenceLog),
        #[prost(message, tag="5")]
        SessionRunLog(super::SessionRunLog),
    }
}
/// Generated client implementations.
pub mod prediction_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// open source marker; do not remove
    /// PredictionService provides access to machine-learned models loaded by
    /// model_servers.
    #[derive(Debug, Clone)]
    pub struct PredictionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PredictionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PredictionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PredictionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PredictionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Classify.
        pub async fn classify(
            &mut self,
            request: impl tonic::IntoRequest<super::ClassificationRequest>,
        ) -> Result<tonic::Response<super::ClassificationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/Classify",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Regress.
        pub async fn regress(
            &mut self,
            request: impl tonic::IntoRequest<super::RegressionRequest>,
        ) -> Result<tonic::Response<super::RegressionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/Regress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Predict -- provides access to loaded TensorFlow model.
        pub async fn predict(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictRequest>,
        ) -> Result<tonic::Response<super::PredictResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/Predict",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// MultiInference API for multi-headed models.
        pub async fn multi_inference(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiInferenceRequest>,
        ) -> Result<tonic::Response<super::MultiInferenceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/MultiInference",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetModelMetadata - provides access to metadata for loaded models.
        pub async fn get_model_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelMetadataRequest>,
        ) -> Result<tonic::Response<super::GetModelMetadataResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/GetModelMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
