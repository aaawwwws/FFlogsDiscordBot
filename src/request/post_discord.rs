use reqwest::Client;

use super::res_json;

pub struct PostDiscord {
    _client: Client,
    _id: Option<u64>,
    _killtype: res_json::JsonBool,
    _name: Option<String>,
}

impl PostDiscord {
    pub fn new(
        client: Client,
        id: Option<u64>,
        killtype: res_json::JsonBool,
        name: Option<String>,
    ) -> Self {
        return Self {
            _client: client,
            _id: id,
            _killtype: killtype,
            _name: name,
        };
    }

    pub async fn new_msg(
        &self,
        id: &str,
        wipe_count: &mut u64,
        datetime: &str,
        fight_id: u64,
        area_name: &str,
        hook_url: &str,
    ) -> anyhow::Result<()> {
        let url = format!("https://ja.fflogs.com/reports/{}#fight={}", id, fight_id);
        let msg = format!(
            "時刻:{} エリア:{} {}ワイプ目:{}",
            datetime, area_name, *wipe_count, url
        );
        let _ = self.send_msg(&msg, hook_url).await?;
        *wipe_count += 1;
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
    pub fn get_killtype(&self) -> &res_json::JsonBool {
        return &self._killtype;
    }
    pub fn get_name(&self) -> Option<String> {
        return self._name.clone();
    }
}
