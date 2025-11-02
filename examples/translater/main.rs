mod continuous_translation_from_file;
mod helpers;

#[tokio::main]
async fn main() {
    // requires MS Azure key for subscription with Cognitive Services enabled
    // for convenience MS subscription key can be put into file read by set_env_vars
    helpers::set_env_vars("/tmp/path_to_subscription_key");
    env_logger::init();

    continuous_translation_from_file::run_example().await;
    // works only on system with properly configured microphone
    // from_microphone::run_example().await;

    // not available in public release yet
    //embedded_recognize_once_async_from_file::run_example().await;
}
