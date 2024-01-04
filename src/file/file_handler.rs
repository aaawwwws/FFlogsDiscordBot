use std::{io, io::Write};

use crate::request::res_json;

pub struct FileHandler;

impl FileHandler {
    pub fn input(msg: &str) -> anyhow::Result<String> {
        println!("{}", msg);
        let mut cin = String::new();
        let _ = io::stdin().read_line(&mut cin).expect("test");
        Ok(cin.trim().to_string())
    }

    pub fn web_hook(file_name: &str, token: Option<String>) -> anyhow::Result<res_json::Konoyonoowari> {
        if let Some(toke) = token {
            //初回起動
            let hook_url = FileHandler::input("webhookのURLを入力してください")?;
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
            return Ok(json)
        } else {
            //二回目以降の起動
            let content = std::fs::read_to_string(&file_name).unwrap();
            let deserialized: res_json::Konoyonoowari = serde_json::from_str(&content).unwrap();
            return Ok(deserialized);
        }
    }
}
