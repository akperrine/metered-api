use std::str::FromStr;

use anyhow::Result;
use axum::{
    body::{Body, BodyDataStream, Bytes},
    extract::Path,
    http::{header, response, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Error, Extension, Router,
};

use axum_macros::debug_handler;
use futures_util::{io::BufWriter, AsyncReadExt};
use image::{codecs::png::PngEncoder, DynamicImage, ImageEncoder, ImageFormat};
use mongodb::{
    bson::{oid::ObjectId, Bson},
    GridFsBucket,
};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;

use crate::db::get_bucket;

pub fn create_route() -> Router {
    Router::new()
        .route("/images/:id", get(get_image_by_id))
        .route("/images", post(dummy_fn))
        .route("/images/:id", delete(dummy_fn))
}

pub async fn dummy_fn() {
    println!("hello world");
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
