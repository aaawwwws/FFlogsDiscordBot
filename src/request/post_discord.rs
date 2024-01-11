use std::sync::Arc;

use reqwest::Client;

use super::res_json::{self, Phases};
#[derive(Debug)]
pub struct PostDiscord {
    _client: Client,
    _id: Option<u64>,
    _killtype: res_json::JsonBool,
    _name: Option<String>,
    _phases: Option<Phases>,
}

impl PostDiscord {
    pub fn new(
        client: Client,
        id: Option<u64>,
        killtype: res_json::JsonBool,
        name: Option<String>,
        phases: Option<Phases>,
    ) -> Self {
        return Self {
            _client: client,
            _id: id,
            _killtype: killtype,
            _name: name,
            _phases: phases,
        };
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

    pub async fn send_file(
        &self,
        content: &str,
        hook_url: Arc<String>,
        file_name: &str,
    ) -> anyhow::Result<u16> {
        let file = reqwest::blocking::multipart::Form::new().file("file", &file_name)?;
        let query = format!(
            r#"{{
            "content": "{}",
            "file": {{
                "name":"{}",
                "content": "{}"
            }}
            }}"#,
            content, file_name, content
        );
        let _ = tokio::task::spawn_blocking(move ||{
            let _res = reqwest::blocking::Client::new()
                .post(hook_url.as_str())
                .multipart(file)
                .send();
        })
        .await?;

        return Ok(22);
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
    pub fn get_phases(&self) -> &Option<Phases> {
        return &self._phases;
    }
}
