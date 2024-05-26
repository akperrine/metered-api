use std::str::FromStr;

use axum::{
    body::Bytes,
    extract::{Multipart, Path},
    http::{header, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};

use axum_macros::debug_handler;
use futures_util::AsyncWriteExt;
use futures_util::{AsyncReadExt, TryStreamExt};
use image::ImageFormat;
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    GridFsBucket,
};
use serde_json::json;

use crate::db::get_bucket;

//TODO: ERROR TODO logic
//1.) Default body limit
//2.) resource not found
//3.) **handle incorrectly sent request
//4.) Add Error handling for each
//5.) Clean up commented out code
//6.) refactor reused code

// TESTs
// Check correct format

pub fn create_route() -> Router {
    Router::new()
        .route("/images/:id", get(get_image_by_id))
        .route("/images/name/:name", get(get_image_by_name))
        .route("/images", post(post_image))
        .route("/images/:id", delete(dummy_fn))
        .route("/images/delete/:id", delete(delete_image_by_id))
}

pub async fn dummy_fn() {
    println!("hello world");
}

#[debug_handler]
pub async fn post_image(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let res = field.name().unwrap() == "file";
        println!("{}", res);

        if field.name().unwrap().eq("file") {
            let name = field.name().unwrap().to_string();
            field.file_name().unwrap();
            if let Some(filename) = field.file_name().map(ToString::to_string) {
                println!(
                    "found file field with name: {}, filename: {}",
                    name, filename
                );
                let bucket = get_bucket().await.unwrap();
                // check if file name already used
                let find_query = doc! {"filename": doc! {"$exists": &filename}};
                let mut cursor = bucket.find(find_query, None).await.unwrap();

                while let Some(_) = cursor.try_next().await.unwrap() {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!(
                            "image name is already taken. Please choose a unique name"
                        )),
                    ));
                }

                let mut data: Vec<u8> = Vec::new();
                while let Some(chunk) = field.chunk().await.unwrap() {
                    data.extend_from_slice(&chunk);
                }

                let mut upload_stream = bucket.open_upload_stream(&filename, None);
                upload_stream.write_all(&data).await.unwrap();
                upload_stream.close().await.unwrap();
            }
        }
    }

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
    let bson_id = Bson::ObjectId(id);
    get_response_from_gridfs(&bucket, bson_id).await
}

#[debug_handler]
pub async fn get_image_by_name(
    Path(image_name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    let bucket = get_bucket().await.unwrap();
    let find_query = doc! {"filename": image_name};
    let mut cursor = bucket.find(find_query, None).await.unwrap();
    while let Some(res) = cursor.try_next().await.unwrap() {
        println!("File: {:?}", res);
        let id = res.id;
        return get_response_from_gridfs(&bucket, id).await;
    }
    Err((
        StatusCode::BAD_REQUEST,
        Json(serde_json::to_vec(&json!({ "message": "Image not found with this name" })).unwrap())
            .to_vec(),
    ))
}

#[debug_handler]
pub async fn delete_image_by_id(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    println!("id: {}", id);
    let bucket = get_bucket().await.unwrap();
    let obj_id = ObjectId::from_str(&id).unwrap();
    println!("object id: {}", &obj_id);
    let result = bucket.delete(Bson::ObjectId(obj_id)).await.unwrap();
    println!("result from delete, {:?}", result);
    let headers = [(header::CONTENT_TYPE, "image/png")];
    Ok((
        headers,
        Json(json!({
            "message": "Image successfully deleted"
        })),
    ))
}

// Common Service functions
pub async fn get_response_from_gridfs(
    bucket: &GridFsBucket,
    id: Bson,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    let mut buffer: Vec<u8> = Vec::new();
    let download_stream = bucket.open_download_stream(id).await;
    match download_stream {
        Ok(mut stream) => {
            stream.read_to_end(&mut buffer).await.unwrap();

            let cursor = std::io::Cursor::new(&mut buffer);
            image::io::Reader::with_format(cursor, ImageFormat::Png)
                .decode()
                .map_err(|e| format!("Failed to decode PNG image: {:?}", e))
                .unwrap();
            let bytes: Bytes = buffer.into();
            let headers = [(header::CONTENT_TYPE, "image/png")];
            Ok((headers, bytes))
        }
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::to_vec(&json!({ "message": "Image id not found" })).unwrap()).to_vec(),
        )),
    }
}
