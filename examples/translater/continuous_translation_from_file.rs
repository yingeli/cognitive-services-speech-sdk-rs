use super::helpers;
use log::*;
use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running continuous_translation_from_file example...");
    info!("---------------------------------------------------");

    let filename = helpers::get_sample_file("chinese_test.wav");

    let mut translation_recognizer = helpers::translation_recognizer_from_wav_file(&filename);

    helpers::set_callbacks(&mut translation_recognizer);

    if let Err(err) = translation_recognizer
        .start_continuous_recognition_async()
        .await
    {
        error!("start_continuous_recognition_async error {:?}", err);
    }
    sleep(Duration::from_millis(10000)).await;
    translation_recognizer
        .stop_continuous_recognition_async()
        .await
        .unwrap();

    info!("example finished!");
}
