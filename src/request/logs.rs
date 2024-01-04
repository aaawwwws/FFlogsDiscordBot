use std::{fs, path::Path};

use crate::{file::file_handler::FileHandler, request::res_json::Object};
pub struct Logs;
impl Logs {
    pub async fn get_token(file_name: &str) -> anyhow::Result<Option<String>> {
        let file_check =
            Path::new(&file_name).exists() && fs::metadata(&file_name).unwrap().is_file();
        if !file_check {
            //初回起動
            let client_id = FileHandler::input("client_idを入力してください")?;
            let client_secret = FileHandler::input("client_secretを入力してください")?;
            let client = reqwest::Client::new();
            let res = client
                .post("https://ja.fflogs.com/oauth/token")
                .basic_auth(client_id, Some(client_secret))
                .form(&[("grant_type", "client_credentials")])
                .send()
                .await?;
            let json: Object = res.json().await?;
            Ok(Some(json.access_token))
        } else {
            //二度目以降
            return Ok(None);
        }
    }
}
