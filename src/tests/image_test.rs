use std::env::temp_dir;

use mongodb::options::ClientOptions;

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

#[tokio::test]
async fn test_post_get_delete_png() {
    println!("BI");
    println!("Not Hit");
    use_test_app(async move {
        assert_eq!(1, 1);
    })
    .await;
    // let db_lists = db.list_collection_names(None).await;
    println!("??");
    // println!("{:?}", db_lists);
    // println!("{:?} hi", db);
}
