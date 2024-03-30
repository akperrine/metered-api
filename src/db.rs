use tokio::sync::OnceCell;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    mongo_url: String,
}

static CONNECTION: OnceCell<Database> = OnceCell::const_new();

pub async fn connection() -> &'static Database {
    let config = get_env_config();

    let mut client_options = ClientOptions::parse_async(config.mongo_url).await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    CONNECTION
        .get_or_init(|| async {
            let client = Client::with_options(client_options).unwrap();

            client.database("images")
        })
        .await
}

fn get_env_config() -> Config {
    let env_vars = std::fs::read_to_string("env.toml").expect("unable to read config file");
    let config: Config = toml::from_str(&env_vars).expect("unable to parse toml file");
    config
}
