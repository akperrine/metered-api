mod app;
mod db;
mod error;
mod models;
mod routes;

use std::str::FromStr;

use futures_util::AsyncReadExt;
use mongodb::bson::{oid::ObjectId, Bson};

use crate::db::{connection, get_bucket};

#[tokio::main]
async fn main() {
    connection().await;

    // GET BUCKET AND PATH
    // let bucket = get_bucket().await.unwrap();
    // let path = "sig.png";

    // DOWNLOAD FILE INFO
    // let id = ObjectId::from_str("661190f4952cdb96750a4405").expect("Could not convert to ObjectId");
    // let mut buf: Vec<u8> = Vec::new();
    // let mut download_stream = bucket
    //     .open_download_stream(Bson::ObjectId(id))
    //     .await
    //     .unwrap();

    // download_stream.read_to_end(&mut buf).await.unwrap();

    // let cursor = std::io::Cursor::new(buf);

    // let img = image::io::Reader::with_format(cursor, image::ImageFormat::Png)
    //     .decode()
    //     .map_err(|e| format!("Failed to decode PNG image: {:?}", e))
    //     .unwrap();

    // let mut output_file = std::fs::File::create("new_sig.png").expect("Unable to create file");
    // img.write_to(&mut output_file, image::ImageFormat::Png)
    //     .unwrap();

    // UPLOAD LOGIC WORKS
    // let img_bytes = fs::read("sig.png").await.unwrap();

    // let mut upload_stream = bucket.open_upload_stream(&path, None);
    // upload_stream.write_all(&img_bytes[..]).await.unwrap();
    // upload_stream.close().await.unwrap();

    let app = app::create_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("local host runing on port 3000");

    axum::serve(listener, app).await.unwrap()
}
