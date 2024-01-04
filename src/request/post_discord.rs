use reqwest::Client;

pub struct PostDiscord {
    _client: Client,
    _id: Option<u64>,
}

impl PostDiscord {
    pub fn new(client: Client, id: Option<u64>) -> Self {
        return Self {
            _client: client,
            _id: id,
        };
    }

    pub async fn new_msg(&self, id: &str, fight_id: u64, hook_url: &str) -> anyhow::Result<()> {
        let url = format!("https://ja.fflogs.com/reports/{}#fight={}", id, fight_id);
        let _ = self.send_msg(&url, hook_url).await?;
        return Ok(());
    }

    pub async fn send_msg(&self, content: &str, hook_url: &str) -> anyhow::Result<u16> {
        let query = format!(
            r#"{{
            "content": "{}"
            }}"#,
            content
        );

        let Ok(res) = self
            ._client
            .post(hook_url)
            .header("Content-Type", "application/json")
            .body(query)
            .send()
            .await
        else {
            return Err(anyhow::anyhow!("接続できませんでした。"));
        };
        return Ok(res.status().as_u16());
    }

    pub fn get_id(&self) -> Option<u64> {
        return self._id;
    }
}
