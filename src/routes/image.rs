use std::fs;
use std::str::FromStr;
use std::{env, time::Duration};

use axum::{
    body::Bytes,
    extract::{multipart, Multipart, Path},
    http::{header, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use axum_macros::debug_handler;
use bson::Document;
use futures_util::AsyncWriteExt;
use futures_util::{AsyncReadExt, TryStreamExt};
use image::ImageFormat;
use mongodb::Collection;
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    options::{GridFsBucketOptions, WriteConcern},
    GridFsBucket,
};
use serde_json::json;

use crate::db::connection;
use crate::models::user::{DtoUser, User};
use crate::{db::get_bucket, models::user::PublicUser};

pub fn create_route() -> Router {
    Router::new()
        .route("/images/:id", get(get_image_by_id))
        .route("/images/name/:name", get(get_image_by_name))
        .route("/images", post(post_image))
        .route(
            "/images/updateProfilePic/:id",
            post(update_user_profile_pic),
        )
        .route("/images/delete/:id", delete(delete_image_by_id))
        .route("/images/removeProfilePic", delete(delete_user_profile_pic))
}

#[debug_handler]
pub async fn post_image(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        // let res = field.name().unwrap() == "file";

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
                let find_query = doc! {"filename": &filename};
                let mut cursor = bucket.find(find_query, None).await.unwrap();

                while let Some(_) = cursor.try_next().await.unwrap() {
                    return Err(error_fmt(
                        StatusCode::BAD_REQUEST,
                        "image name is already taken. Please choose a unique name",
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
        } else {
            return Err(error_fmt(
                StatusCode::BAD_REQUEST,
                "multipart form data requires field name: file with key as the loaded file",
            ));
        }
    }

    let success_response = Json(json!({
        "message": "Image successfully loaded"
    }));
    Ok((StatusCode::OK, success_response))
}

#[debug_handler]
pub async fn get_image_by_id(
    _user: PublicUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    let bucket = get_bucket().await.unwrap();
    let id = ObjectId::from_str(&id).expect("could not convert id to ObjectId");
    let bson_id = Bson::ObjectId(id);
    get_response_from_gridfs(&bucket, bson_id).await
}

#[debug_handler]
pub async fn get_image_by_name(
    _user: PublicUser,
    Path(image_name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    let bucket = get_bucket().await.unwrap();
    let find_query = doc! {"filename": image_name};
    let mut cursor = bucket.find(find_query, None).await.unwrap();
    while let Some(res) = cursor.try_next().await.unwrap() {
        let id = res.id;
        return get_response_from_gridfs(&bucket, id).await;
    }
    Err(error_fmt(
        StatusCode::BAD_REQUEST,
        "Image not found with this name",
    ))
}

#[debug_handler]
pub async fn delete_image_by_id(
    _user: PublicUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    let bucket = get_bucket().await.unwrap();
    let obj_id = ObjectId::from_str(&id).unwrap();
    bucket.delete(Bson::ObjectId(obj_id)).await.unwrap();
    let headers = [(header::CONTENT_TYPE, "image/png")];
    Ok((
        headers,
        Json(json!({
            "message": "Image successfully deleted"
        })),
    ))
}

#[debug_handler]
async fn update_user_profile_pic(
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    let connection = connection().await;
    let write_concern = WriteConcern::builder()
        .w_timeout(Duration::new(5, 0))
        .build();
    let options = GridFsBucketOptions::builder()
        .bucket_name("image_bucket".to_string())
        .write_concern(write_concern)
        .build();
    let bucket = connection.gridfs_bucket(options);
    let obj_id = ObjectId::from_str(&id).unwrap();
    // disgard the error of the
    bucket.delete(Bson::ObjectId(obj_id)).await.ok();
    // If does update, else insert
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let res = field.name().unwrap() == "file";

        if field.name().unwrap().eq("file") {
            let name = field.name().unwrap().to_string();
            field.file_name().unwrap();
            if let Some(mut filename) = field.file_name().map(ToString::to_string) {
                filename = String::from(&id);
                println!(
                    "found file field with name: {}, filename: {} to",
                    name, filename
                );
                // check if file name already used
                let mut data: Vec<u8> = Vec::new();
                while let Some(chunk) = field.chunk().await.unwrap() {
                    data.extend_from_slice(&chunk);
                }

                let mut upload_stream = bucket.open_upload_stream(&filename, None);
                upload_stream.write_all(&data).await.unwrap();
                upload_stream.close().await.unwrap();
            }
            let updated_name = String::from(&id.to_string());
            let collection: Collection<User> = connection.collection("users");
            let bson = Bson::ObjectId(ObjectId::from_str(&id).unwrap());
            let mut res = collection.find(doc! {"_id": &bson}, None).await.unwrap();
            while let Some(item) = res.try_next().await.unwrap() {
                println!("{:?}, alkfj, {:?}", item, &updated_name);
            }
            println!("sdflKJ {:?}", &bson);
            let update = collection
                .update_one(
                    doc! {"_id": &bson},
                    doc! {"$set": {"profile_pic_url": updated_name}},
                    None,
                )
                .await
                .unwrap();

            println!("{:?}", update);
        } else {
            return Err(error_fmt(
                StatusCode::BAD_REQUEST,
                "multipart form data requires field name: file with key as the loaded file",
            ));
        }
    }

    let success_response = Json(json!({
        "message": "Image successfully loaded"
    }));
    Ok((StatusCode::OK, success_response))
}

#[debug_handler]
async fn delete_user_profile_pic(Json(user): Json<DtoUser>) {
    let connection = connection().await;
    let write_concern = WriteConcern::builder()
        .w_timeout(Duration::new(5, 0))
        .build();
    let options = GridFsBucketOptions::builder()
        .bucket_name("image_bucket".to_string())
        .write_concern(write_concern)
        .build();
    let bucket = connection.gridfs_bucket(options);
    // let fs_connection: Collection<Document> = connection.collection("fs");
    // let mut cursor = fs_connection
    //     .find(doc! {"filename": &user.profile_pic_url}, None)
    //     .await
    //     .unwrap();
    println!("HIDID {:?}", user.profile_pic_url);

    let mut cursor = bucket
        .find(doc! {"filename": user.profile_pic_url}, None)
        .await
        .unwrap();
    while let Some(item) = cursor.try_next().await.unwrap() {
        println!("{:?}", item);
        let id = item.id;
        bucket.delete(id).await.unwrap();
        //TODO: see if delete and then update user profile
    }

    let collection: Collection<User> = connection.collection("users");
    let updated2 = collection.update_one(
        doc! {"_id": &user.id},
        doc! {"$set": {"profile_pic_url": "default_profile.png"}},
        None,
    );
    println!("updated2 {:?}", updated2.await.unwrap());
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
        Err(_) => Err(error_fmt(StatusCode::BAD_REQUEST, "Image id not found")),
    }
}

fn error_fmt(status_code: StatusCode, message: &str) -> (StatusCode, Vec<u8>) {
    (
        status_code,
        Json(serde_json::to_vec(&json!({ "message": message })).unwrap()).to_vec(),
    )
}
