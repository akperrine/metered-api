use std::{
    convert::Infallible,
    fs::File,
    io::{BufWriter, Read, Write},
    str::FromStr,
};

use axum::{
    body::Body,
    extract::{Multipart, Path},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use base64::Engine;
use bson::{doc, oid::ObjectId, Document};
use bytes::Bytes;
use futures::{io::Cursor, TryStreamExt};
use mime::Mime;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_util::io::ReaderStream;

use crate::db::connection;

pub fn create_route() -> Router {
    Router::new()
        .route("/files/:id", get(get_file_by_id))
        .route("/files/view/:file_id", get(get_file_as_view))
        .route("/files/add", post(post_new_file))
        .route("/files/remove", delete(delete_file))
}

pub async fn get_file_by_id(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    let db = connection().await;
    let collection: Collection<AppFile> = db.collection("files");
    println!("{:?}", id);
    let obj_id = ObjectId::from_str(&id).expect("could not convert id to ObjectId");

    let mut cursor = collection.find(doc! {"_id": obj_id}, None).await.unwrap();

    while let Some(file) = cursor.try_next().await.unwrap() {
        let streamBody = Body::from(file.data);

        let headers = [(header::CONTENT_TYPE, "application/pdf")];

        return Ok((headers, streamBody));
    }

    Err(error_fmt(
        StatusCode::BAD_REQUEST,
        "Request not processed correctly",
    ))
}

pub async fn get_file_as_view() {}

pub async fn post_new_file(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap().eq("file") {
            println!("{:?}", field.file_name().unwrap());
            // println!("{:?}", field.bytes().await);
            let db = connection().await;
            let collection: Collection<AppFile> = db.collection("files");
            let file_to_insert = AppFile {
                name: field.file_name().unwrap().to_string(),
                data: field.bytes().await.unwrap().to_vec(),
            };

            let res = collection.insert_one(file_to_insert, None);

            println!("{:?}", res.await.unwrap());

            return Ok((StatusCode::CREATED, Json("user successfully loaded")));
        }
    }
    return Err(error_fmt(
        StatusCode::BAD_REQUEST,
        "Request not processed. Ensure file key is used with file data",
    ));
}

pub async fn delete_file() {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppFile {
    pub name: String,
    pub data: Vec<u8>,
}

fn error_fmt(status_code: StatusCode, message: &str) -> (StatusCode, Vec<u8>) {
    (
        status_code,
        Json(serde_json::to_vec(&json!({ "message": message })).unwrap()).to_vec(),
    )
}
