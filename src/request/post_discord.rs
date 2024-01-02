use reqwest::Client;
use serde_json::json;

pub struct PostDiscord {
    _client: Client,
    _id: u64,
}

impl PostDiscord {
    pub fn new(client: Client, id: u64) -> Self {
        return Self {
            _client: client,
            _id: id,
        };
    }

    pub async fn send_msg(&self, id: &str, fight_id: u64) -> anyhow::Result<()> {
        let url = format!("https://ja.fflogs.com/reports/{}#fight={}", id, fight_id);

        let query = format!(
            r#"{{
            "content": "{}"
        }}"#,
            url
        );

        let _ = self
            ._client
            .post(dotenv::var("HOOK").unwrap())
            .header("Content-Type", "application/json")
            .body(query)
            .send()
            .await;
        return Ok(());
    }

    pub fn get_id(&self) -> u64 {
        return self._id;
    }
}
