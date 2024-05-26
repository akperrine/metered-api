use std::env::{self};

use reqwest::{multipart::Form, Body, Client, Response, StatusCode};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{db::get_env_config, tests::setup::use_test_app};
use bson::oid::ObjectId;

// #[test]
// fn answer_42() {
//     assert_eq!(42, 42);
// }
// #[tokio::test]
// async fn test_connection() {
//     let config = get_env_config();

//     let mut client_options = ClientOptions::parse_async(config.mongo_url).await.unwrap();
//     let client = mongodb::Client::with_options(client_options);
//     print!("HI");

//     assert!(client.is_ok());
// }
#[cfg(test)]
#[test]
fn test_post_get_delete_png() {
    use futures::TryStreamExt;
    use mongodb::bson::{doc, oid::ObjectId, Document};

    use crate::db::get_bucket;

    use_test_app(async move {
        println!("{}", env::current_dir().unwrap().display());
        // let test_img = File::open("./src/tests/images/test_img_1.png")
        //     .await
        //     .map_err(|_| "Cannot find test image")
        //     .unwrap();

        // // read file body stream
        // let stream = FramedRead::new(test_img, BytesCodec::new());
        // let file_body = Body::wrap_stream(stream);
        // //make form part of a file
        // let some_file = reqwest::multipart::Part::stream(file_body)
        //     .file_name("test_img_1.png")
        //     .mime_str("image/png")
        //     .unwrap();
        // // create multipart form
        // let form = reqwest::multipart::Form::new().part("file", some_file);
        let form = create_multipart_form().await;

        let client = Client::new();
        let res = client
            .post("http://localhost:3001/images")
            .multipart(form)
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let expected = StatusCode::OK;
        println!("{},{}", status_code, expected);
        assert_eq!(status_code, expected);

        // get id
        let bucket = get_bucket().await.unwrap();
        // let mut id: String = String::from("none");
        let mut id: String = String::from("none");
        // check if file name already used
        let find_query = doc! {"filename": "test_img_1.png"};
        let mut cursor = bucket.find(find_query, None).await.unwrap();

        while let Some(res) = cursor.try_next().await.unwrap() {
            // println!("id: {}", res.id.to_string().h);

            id = res.id.as_object_id().unwrap().to_hex();

            println!("id: {}", id);
        }
        println!("id: {}", id);

        let get_right_id = client
            .get(format!("http://localhost:3001/images/{}", &id))
            .send()
            .await
            .unwrap();

        let get_correct_id_code = get_right_id.status();
        let expected = StatusCode::OK;
        println!("{},{}", get_correct_id_code, expected);
        assert_eq!(get_correct_id_code, expected);

        // check twice on posting same: should fail
        let form_copy = create_multipart_form().await;
        let res_two = client
            .post("http://localhost:3001/images")
            .multipart(form_copy)
            .send()
            .await
            .unwrap();
        let status_code = res_two.status();
        let resp_message = res_two.text().await.unwrap();
        let expected = StatusCode::BAD_REQUEST;
        println!("{},{}", status_code, expected);
        // println!("{}", res_two.text().await.unwrap());
        assert_eq!(
            resp_message,
            "\"image name is already taken. Please choose a unique name\""
        );
        assert_eq!(status_code, expected);

        let get_right_res = client
            .get("http://localhost:3001/images/name/test_img_1.png")
            .send()
            .await
            .unwrap();

        let get_correct_status_code = get_right_res.status();
        let expected = StatusCode::OK;
        println!("{},{}", get_correct_status_code, expected);
        assert_eq!(get_correct_status_code, expected);

        let get_wrong_res = client
            .get("http://localhost:3001/images/name/wrong_name.png")
            .send()
            .await
            .unwrap();

        let get_wrong_status_code = get_wrong_res.status();
        let expected = StatusCode::BAD_REQUEST;
        println!("{},{}", get_wrong_status_code, expected);
        assert_eq!(status_code, expected);

        let delete_res = client
            .delete(format!("http://localhost:3001/images/delete/{}", &id))
            .send()
            .await
            .unwrap();

        let delete_status_code = delete_res.status();
        let expected = StatusCode::OK;
        println!("Delete Status codes: {},{}", delete_status_code, expected);
        assert_eq!(delete_status_code, expected);
    });
}

async fn create_multipart_form() -> Form {
    let test_img = File::open("./src/tests/images/test_img_1.png")
        .await
        .map_err(|_| "Cannot find test image")
        .unwrap();

    // read file body stream
    let stream = FramedRead::new(test_img, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);
    //make form part of a file
    let some_file = reqwest::multipart::Part::stream(file_body)
        .file_name("test_img_1.png")
        .mime_str("image/png")
        .unwrap();
    // create multipart form
    let form = reqwest::multipart::Form::new().part("file", some_file);
    return form;
    // // read file body stream
    // let stream = FramedRead::new(file, BytesCodec::new());
    // let file_body = Body::wrap_stream(stream);
    // // make form part of a file
    // let some_file = reqwest::multipart::Part::stream(file_body)
    //     .file_name("test_img_1.png")
    //     .mime_str("image/png")
    //     .unwrap();
    // // create multipart form
    // let form = reqwest::multipart::Form::new().part("file", some_file);

    // let client = Client::new();

    // client
    //     .post("http://localhost:3001/images")
    //     .multipart(form)
    //     .send()
    //     .await
    //     .unwrap()
}
