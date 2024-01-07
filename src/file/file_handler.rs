use std::{fs, io, io::Write, path::Path};

use reqwest::Client;

use crate::request::{
    self,
    res_json::{self, JsonBool},
};

pub struct FileHandler;

impl FileHandler {
    pub fn input(msg: &str) -> anyhow::Result<String> {
        println!("{}", msg);
        let mut cin = String::new();
        let _ = io::stdin().read_line(&mut cin).expect("test");
        Ok(cin.trim().to_string())
    }

    pub async fn web_hook(
        file_name: &str,
        token: Option<String>,
    ) -> anyhow::Result<res_json::Konoyonoowari> {
        if let Some(toke) = token {
            //初回起動
            let hook_url = loop {
                let hook_url = FileHandler::input("webhookのURLを入力してください")?;
                println!("接続を確認します。");
                let discord = request::post_discord::PostDiscord::new(
                    Client::new(),
                    None,
                    JsonBool::NULL,
                    None,
                );
                if let Ok(_) = discord.send_msg("成功", &hook_url).await {
                    break hook_url;
                } else {
                    ()
                };
            };
            let mut fout = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&file_name)
                .unwrap();
            let json = res_json::Konoyonoowari {
                key: toke,
                webhook: hook_url,
            };
            let serialized = serde_json::to_string(&json).unwrap();
            let _ = fout.write_all(serialized.as_bytes());
            return Ok(json);
        } else {
            //二回目以降の起動
            let content = std::fs::read_to_string(&file_name).unwrap();
            let deserialized: res_json::Konoyonoowari = serde_json::from_str(&content).unwrap();
            return Ok(deserialized);
        }
    }

    pub fn wipe_count(wipe_count: u64) -> anyhow::Result<()> {
        let file_name = String::from("wipe_count.txt");
        let file_check =
            Path::new(&file_name).exists() && fs::metadata(&file_name).unwrap().is_file();
        if file_check {
            //ファイルがある場合の処理
            let file = std::fs::read_to_string(&file_name)?;
            let count = file.parse::<u64>().unwrap();
            let total_wipe = count + wipe_count;
            let _ = std::fs::write(&file_name, total_wipe.to_string())?;
        } else {
            //ない場合の処理
            let _ = std::fs::File::create(&file_name)?;
        }
        return Ok(());
    }
}
