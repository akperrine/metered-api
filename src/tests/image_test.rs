use std::env::{self};

use reqwest::{multipart::Form, Body, Client, Response, StatusCode};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{db::get_env_config, tests::setup::use_test_app};

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
