use std::{
    convert::Infallible,
    fs::File,
    io::{BufWriter, Read, Write},
    mem::MaybeUninit,
    str::FromStr,
};

use axum::{
    body::{Body, HttpBody},
    extract::{Multipart, Path},
    http::{header, Error, Response, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use base64::Engine;
use bson::{doc, oid::ObjectId, Document};
use bytes::Bytes;
use futures::{io::Cursor, TryStreamExt};
use mime::Mime;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_util::io::ReaderStream;

use crate::{db::connection, models::user::User};

pub fn create_route() -> Router {
    Router::new()
        .route("/files/:id", get(get_file_by_id))
        .route("/files/view", get(get_file_as_view))
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

pub async fn get_file_as_view(
    body: Json<ViewRequestBody>,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    let db = connection().await;
    let mut owner = find_query(&db, &body.owner_id).await.unwrap();
    let mut viewer = find_query(&db, &body.viewer_id).await.unwrap();
    println!("{:?}", viewer.account.bill);
    //get file
    let collection: Collection<AppFile> = db.collection("files");
    // messy and needs refactoring... just burning out on project
    let user_collection: Collection<User> = db.collection("users");
    let obj_id = ObjectId::from_str(&body.file_id).unwrap();
    let mut cursor = collection.find(doc! {"_id": obj_id}, None).await.unwrap();
    while let Some(file) = cursor.try_next().await.unwrap() {
        let update_owner = user_collection
            .update_one(
                doc! {"_id": ObjectId::from_str(&body.owner_id).unwrap()},
                doc! {"$set": {"account.earnings":  owner.account.earnings + 1}},
                None,
            )
            .await
            .unwrap();
        let update_viewer = user_collection
            .update_one(
                doc! {"_id": ObjectId::from_str(&body.viewer_id).unwrap()},
                doc! {"$set": {"account.bill":  viewer.account.bill + 1}},
                None,
            )
            .await
            .unwrap();
        println!("{:?}, {:?}", update_owner, update_viewer);
        // println!("owner: {:?}, viewer: {:?}", owner, viewer);
        let streamBody = Body::from(file.data);
        let headers = [(header::CONTENT_TYPE, "application/pdf")];

        return Ok((headers, streamBody));
    }

    Err(error_fmt(
        StatusCode::BAD_REQUEST,
        "Request not processed correctly",
    ))
}

pub async fn post_new_file(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Vec<u8>)> {
    let mut file_to_insert = AppFile {
        name: String::from(""),
        data: vec![],
        owner_id: ObjectId::new(),
    };
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap().eq("file") {
            println!("{:?}", field.file_name().unwrap());
            // println!("{:?}", field.bytes().await);

            file_to_insert.name = field.file_name().unwrap().to_string();
            file_to_insert.data = field.bytes().await.unwrap().to_vec();
        } else if field.name().unwrap().eq("owner_id") {
            // println!("{:?}", field.text().await.unwrap());
            file_to_insert.owner_id =
                ObjectId::from_str(field.text().await.unwrap().as_str()).unwrap();
        }
        // this should check if a file exists with that name first
        // let res = collection.insert_one(file_to_insert, None);

        // println!("{:?}", res.await.unwrap());
    }
    let db = connection().await;
    println!("{:?}", file_to_insert.owner_id);
    let collection: Collection<AppFile> = db.collection("files");
    let res = collection.insert_one(file_to_insert, None).await.unwrap();
    return Ok((StatusCode::CREATED, Json("user successfully loaded")));

    return Err(error_fmt(
        StatusCode::BAD_REQUEST,
        "Request not processed. Ensure file key is used with file data",
    ));
}

pub async fn delete_file() {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppFile {
    pub name: String,
    pub owner_id: ObjectId,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewRequestBody {
    file_id: String,
    owner_id: String,
    viewer_id: String,
}

fn error_fmt(status_code: StatusCode, message: &str) -> (StatusCode, Vec<u8>) {
    (
        status_code,
        Json(serde_json::to_vec(&json!({ "message": message })).unwrap()).to_vec(),
    )
}

async fn find_query(db: &Database, id: &str) -> Result<User, Box<dyn std::error::Error>> {
    println!("hii, {:?}", id);
    let collection: Collection<User> = db.collection("users");
    let obj_id = ObjectId::from_str(id).unwrap();
    let mut cursor = collection.find(doc! {"_id": obj_id}, None).await.unwrap();
    while let Some(user) = cursor.try_next().await.unwrap_or_default() {
        println!("{:?}", user);
        return Ok(user);
        // println!("{:?}", &res);
        // let streamBody = Body::from(file.data);
        // let headers = [(header::CONTENT_TYPE, "application/pdf")];
    }
    Err("User not found")?
}
