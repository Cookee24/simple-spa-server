#[cfg(feature = "bundle")]
use rust_embed::RustEmbed;

#[cfg(feature = "bundle")]
use mime_guess;

#[cfg(feature = "bundle")]
#[derive(RustEmbed)]
#[folder = "www/"]
pub struct StaticFiles;

#[cfg(feature = "bundle")]
pub fn get_embedded_file(path: &str) -> Option<Vec<u8>> {
    let path = if path == "/" || path.is_empty() {
        "index.html"
    } else {
        path.trim_start_matches('/')
    };

    StaticFiles::get(path).map(|f| f.data.into())
}

#[cfg(feature = "bundle")]
pub fn get_embedded_file_type(path: &str) -> String {
    let guess = mime_guess::from_path(path).first_or_text_plain();
    guess.to_string()
}
