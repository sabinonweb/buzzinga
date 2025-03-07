pub fn build_audio_url(fallback_url: String) -> String {
    let url_segments = fallback_url.split("/").collect::<Vec<&str>>();
    format!("https://v.redd.it/{}/DASH_AUDIO_128.mp4", url_segments[3])
}
