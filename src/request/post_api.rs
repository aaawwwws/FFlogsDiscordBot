use std::ptr::null;

use crate::request::res_json::{Charas, Phases, ResJson};
use reqwest::Client;
use serde_json::{Map, Value};

use super::post_discord::PostDiscord;

pub async fn last_fights(id: &str, key: &str, ty: Type) -> anyhow::Result<PostDiscord> {
    let client = Client::new();
    let mut map = Map::new();

    let query = match ty {
        Type::KILL => {
            format!(
                "query {{ reportData {{ report(code:\"{}\") {{ rankings }} }} }}",
                id
            )
        }
        Type::WIPE => {
            format!(
            "query {{ reportData {{ report(code:\"{}\") {{fights {{ id, kill, name, phaseTransitions {{ id }} }} }} }} }}",
            id
        )
        }
    };
    map.insert("query".to_owned(), Value::String(query));
    let json = Value::Object(map);
    let json_string = serde_json::to_string(&json).unwrap();
    let res: ResJson = client
        .post("https://www.fflogs.com/api/v2/client")
        .bearer_auth(key)
        .header("Content-Type", "application/json")
        .body(json_string)
        .send()
        .await?
        .json::<ResJson>()
        .await?;
    let post = match res.get_figths() {
        Some(res) => {
            let res_data = res.iter().last().unwrap();
            let post = PostDiscord::new(
                client,
                Some(res_data.get_id()),
                res_data.get_killtype(),
                res_data.get_name(),
                res_data.get_phases()?,
                None,
            );
            post
        }
        None => {
            let post = PostDiscord::new(
                client,
                None,
                super::res_json::JsonBool::NULL,
                None,
                None,
                res.get_rankig_role().cloned(),
            );
            post
        }
    };
    Ok(post)
}

pub enum Type {
    KILL,
    WIPE,
}
