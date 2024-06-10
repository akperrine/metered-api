use futures::TryStreamExt;
use reqwest::{multipart::Form, Body, Client, Response, StatusCode};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    db::get_bucket,
    routes::user::{AuthResponse, CreateUserBody, LoginUserBody},
    tests::setup::use_test_app,
};
use bson::doc;

// to log test: `cargo test -- --nocapture`
#[test]
fn test_e2e_image_png() {
    use_test_app(async move {
        println!("\nInit end to end Test for images:\n");
        let client = Client::new();

        let login_body = LoginUserBody {
            email: String::from("user@gmail.com"),
            password: String::from("123123"),
        };
        let create_user_body = CreateUserBody {
            username: String::from("user"),
            email: String::from("user@gmail.com"),
            password: String::from("123123"),
        };

        println!("login before create: should fail 401 Unauthorized");
        let failed_login_res = client
            .get("http://localhost:3001/users/login")
            .json(&login_body)
            .send()
            .await
            .unwrap();
        check_status(&failed_login_res, StatusCode::UNAUTHORIZED);

        println!("create user: should pass 201 Created");
        let create_user_res = client
            .post("http://localhost:3001/users/create")
            .json(&create_user_body)
            .send()
            .await
            .unwrap();
        check_status(&create_user_res, StatusCode::CREATED);

        println!("login: should pass 200 OK");
        let login_res = client
            .get("http://localhost:3001/users/login")
            .json(&login_body)
            .send()
            .await
            .expect("failed to get response");
        check_status(&login_res, StatusCode::OK);

        let payload = login_res
            .json::<AuthResponse>()
            .await
            .expect("failed to get payload");
        // println!("{:?}", login_res);
        let auth_token = payload.access_token;

        // TODO: need to get bearer tokens here
        println!("Posting image: should pass: 200 OK");
        let form = create_multipart_form().await;
        let post_first_res = client
            .post("http://localhost:3001/images")
            .multipart(form)
            .bearer_auth(&auth_token)
            .send()
            .await
            .expect("failed to get response");
        check_status(&post_first_res, StatusCode::OK);

        // retrieve id from just posted image
        let id = get_image_id().await.unwrap();

        println!("Getting image by id: should return: 200 Ok");
        let get_exists_res = client
            .get(format!("http://localhost:3001/images/{}", &id))
            .bearer_auth(&auth_token)
            .send()
            .await
            .unwrap();
        check_status(&get_exists_res, StatusCode::OK);

        println!("Re-posting same image: should return: 400 Bad Request");
        let form_copy = create_multipart_form().await;
        let post_repeat_res = client
            .post("http://localhost:3001/images")
            .multipart(form_copy)
            .bearer_auth(&auth_token)
            .send()
            .await
            .unwrap();
        check_status(&post_repeat_res, StatusCode::BAD_REQUEST);
        let resp_message = post_repeat_res.text().await.unwrap();
        assert_eq!(
            resp_message,
            "{\"message\":\"image name is already taken. Please choose a unique name\"}"
        );

        println!("Getting image by name: should return: 200 Ok");
        let get_right_res = client
            .get("http://localhost:3001/images/name/test_img_1.png")
            .bearer_auth(&auth_token)
            .send()
            .await
            .unwrap();
        check_status(&get_right_res, StatusCode::OK);

        println!("Getting image by WRONG name: should return: 400 Bad Request");
        let get_wrong_res = client
            .get("http://localhost:3001/images/name/wrong_name.png")
            .bearer_auth(&auth_token)
            .send()
            .await
            .unwrap();
        check_status(&get_wrong_res, StatusCode::BAD_REQUEST);

        println!("Delete image should return: 200 Ok");
        let delete_res = client
            .delete(format!("http://localhost:3001/images/delete/{}", &id))
            .bearer_auth(&auth_token)
            .send()
            .await
            .unwrap();
        check_status(&delete_res, StatusCode::OK);

        println!("Getting image by right name After Delete: should return: 400 Bad Request");
        let get_right_id_after_delete = client
            .get(format!("http://localhost:3001/images/{}", &id))
            .bearer_auth(&auth_token)
            .send()
            .await
            .unwrap();
        check_status(&get_right_id_after_delete, StatusCode::BAD_REQUEST);
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
}

fn check_status(res: &Response, expected_status: StatusCode) {
    let status_code = res.status();
    assert_eq!(status_code, expected_status);
    println!(
        "status code: {}, expected: {} \n",
        status_code, expected_status
    );
}

async fn get_image_id() -> Result<String, std::fmt::Error> {
    let bucket = get_bucket().await.unwrap();
    // placeholder String: test fails anyways if not replaced
    let mut id: String = String::from("none");
    // check if file name already used
    let find_query = doc! {"filename": "test_img_1.png"};
    let mut cursor = bucket.find(find_query, None).await.unwrap();
    while let Some(res) = cursor.try_next().await.unwrap() {
        id = res.id.as_object_id().unwrap().to_hex();
    }
    Ok(id)
}
