use std::path::PathBuf;
use rocket::fs::NamedFile;

// this module for user
pub mod users;

/**
 * this for front end SPA routing.
 */
#[get("/<_..>", rank = 2)]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

/**
 * this for SPA static serving.
 */
#[get("/<file..>", rank = 1)]
pub async fn static_files(file: PathBuf) -> Option<NamedFile> {
    println!("here");

    let file_name = file.to_str().expect("msg");
    let file_path = format!("static/assets/{:?}", file_name).replace("\"", "");

    NamedFile::open(file_path).await.ok()
}
