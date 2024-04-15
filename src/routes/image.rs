use std::{
    fmt::{self, Formatter},
    io::Write,
    path::Path as StdPath,
    str::FromStr,
};

use axum::{
    body::Bytes,
    extract::{Multipart, Path},
    http::{header, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};

use axum_macros::debug_handler;
use futures_util::{stream::StreamExt, AsyncWriteExt};
use futures_util::{AsyncReadExt, TryStreamExt};
use image::ImageFormat;
use mongodb::bson::{oid::ObjectId, Bson};
use serde_json::json;
use tokio::fs;

use crate::db::get_bucket;

//TODO: ERROR TODO logic
//1.) Default body limit
//2.) resource not found
//3.) **handle incorrectly sent request

pub fn create_route() -> Router {
    Router::new()
        .route("/images/:id", get(get_image_by_id))
        .route("/images", post(post_image))
        .route("/images/:id", delete(dummy_fn))
}

pub async fn dummy_fn() {
    println!("hello world");
}

#[debug_handler]
pub async fn post_image(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        // let data = field.bytes().await.unwrap();
        let fieldName = field.file_name().unwrap();
        if let Some(filename) = field.file_name().map(ToString::to_string) {
            println!(
                "found file field with name: {}, filename: {}",
                name, filename
            );

            let mut data: Vec<u8> = Vec::new();
            // let mut file = std::fs::File::create(StdPath::new(&filename))
            //     .map_err(|error| "error opening the file path to write")
            //     .unwrap();
            while let Some(chunk) = field.chunk().await.unwrap() {
                data.extend_from_slice(&chunk);
            }

            let bucket = get_bucket().await.unwrap();
            let mut upload_stream = bucket.open_upload_stream(&filename, None);
            upload_stream.write_all(&data).await.unwrap();
            upload_stream.close().await.unwrap();

            // match fs::remove_file(&filename).await {
            //     Ok(()) => println!("File deleted successfully."),
            //     Err(err) => println!("Error deleting file: {}", err),
            // }
        }
    }
    // TODO: Take in multipart/form-data with img bytes and name
    // TODO: Get the bin data read up into GridFs

    let success_response = Json(json!({
        "message": "Image successfully loaded"
    }));
    Ok((StatusCode::OK, success_response))
}

#[debug_handler]
pub async fn get_image_by_id(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    println!("{}", id);
    let bucket = get_bucket().await.unwrap();
    let id = ObjectId::from_str(&id).expect("could not convert id to ObjectId");
    let mut buffer: Vec<u8> = Vec::new();
    let mut download_stream = bucket
        .open_download_stream(Bson::ObjectId(id))
        .await
        .unwrap();
    let result = download_stream.read_to_end(&mut buffer).await.unwrap();

    let cursor = std::io::Cursor::new(&mut buffer);
    let img = image::io::Reader::with_format(cursor, ImageFormat::Png)
        .decode()
        .map_err(|e| format!("Failed to decode PNG image: {:?}", e))
        .unwrap();

    let bytes: Bytes = buffer.into();

    let headers = [(header::CONTENT_TYPE, "image/png")];

    Ok((headers, bytes))
}

#[derive(Debug)]
struct ProcessImageError(String);

impl fmt::Display for ProcessImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ProcessImageError: {}", self.0)
    }
}

impl std::error::Error for ProcessImageError {}

// let id = ObjectId::from_str("661190f4952cdb96750a4405").expect("Could not convert to ObjectId");
// let mut buf: Vec<u8> = Vec::new();
// let mut download_stream = bucket
//     .open_download_stream(Bson::ObjectId(id))
//     .await
//     .unwrap();

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
