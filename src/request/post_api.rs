use crate::request::res_json::ResJson;
use reqwest::Client;
use serde_json::{Map, Value};

use super::post_discord::PostDiscord;

pub async fn last_fight(id: &str) -> anyhow::Result<PostDiscord> {
    let client = Client::new();
    let mut map = Map::new();
    let query = format!(
        "query {{ reportData {{ report(code:\"{}\") {{ fights {{ id }} }} }} }}",
        id
    );
    map.insert("query".to_owned(), Value::String(query));
    let json = Value::Object(map);
    let json_string = serde_json::to_string(&json).unwrap();
    let res: ResJson = client
        .post("https://www.fflogs.com/api/v2/client")
        .bearer_auth(dotenv::var("KEY").unwrap())
        .header("Content-Type", "application/json")
        .body(json_string)
        .send()
        .await?
        .json::<ResJson>()
        .await?;
    let post = PostDiscord::new(client,res.get_figths().iter().last().unwrap().get_id());
    Ok(post)
}
