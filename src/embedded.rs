use std::borrow::Cow;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "www/"]
pub struct StaticFiles;

pub fn get_file(path: &str) -> Option<(Cow<'static, [u8]>, String)> {
    let path = path.trim_start_matches('/');
    StaticFiles::get(path).map(|file| (file.data, file.metadata.mimetype().into()))
}

pub fn get_index_html(path: &str) -> Option<(Cow<'static, [u8]>, String)> {
    let path = path.trim_start_matches('/');
    let index_path = if path.is_empty() {
        "index.html"
    } else {
        &format!("{}/index.html", path)
    };

    StaticFiles::get(index_path).map(|file| (file.data, file.metadata.mimetype().into()))
}
