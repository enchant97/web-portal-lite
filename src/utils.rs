use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

/// Will try to load icon from svg folder then png.
pub async fn load_named_icon(icon_name: &str, icons_path: &PathBuf) -> Option<NamedFile> {
    let svg_path = Path::new(icons_path).join("svg").join(String::from(icon_name) + ".svg");
    let png_path = Path::new(icons_path).join("png").join(String::from(icon_name) + ".png");

    let svg_file = NamedFile::open(svg_path).await.ok();

    match svg_file {
        Some(_) => svg_file,
        None => NamedFile::open(png_path).await.ok(),
    }
}
