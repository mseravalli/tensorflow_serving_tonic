fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src")
        .include_file("lib.rs")
        .compile(
            &[
                "tensorflow_serving/apis/classification.proto",
                "tensorflow_serving/apis/get_model_metadata.proto",
                "tensorflow_serving/apis/get_model_status.proto",
                "tensorflow_serving/apis/inference.proto",
                "tensorflow_serving/apis/input.proto",
                "tensorflow_serving/apis/logging.proto",
                "tensorflow_serving/apis/model_management.proto",
                "tensorflow_serving/apis/model.proto",
                "tensorflow_serving/apis/model_service.proto",
                "tensorflow_serving/apis/prediction_log.proto",
                "tensorflow_serving/apis/prediction_service.proto",
                "tensorflow_serving/apis/predict.proto",
                "tensorflow_serving/apis/regression.proto",
                "tensorflow_serving/apis/session_service.proto",
                "tensorflow_serving/apis/status.proto",
            ],
            &["./"],
        )?;

    Ok(())
}
