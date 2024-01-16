use std::{
    collections::{hash_map, HashMap},
    fs::{self, File, OpenOptions},
    io::Write,
    io::{self, Read},
    path::Path,
};

use base64::read;
use reqwest::{blocking::get, Client};

use crate::{
    file::{self, wipe_data::WipeData},
    request::{
        self,
        res_json::{self, JsonBool},
    },
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
                    None,
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

    pub fn wipe_count(wipe_data: &WipeData) -> anyhow::Result<()> {
        let file_name = String::from("wipe_count.json");
        //ファイルの処理
        if FileHandler::is_file(&file_name) {
            //ファイルがある場合
            let mut read_file = fs::OpenOptions::new().read(true).open(&file_name)?;
            let mut json = String::new();
            let _ = read_file.read_to_string(&mut json);
            let mut json_file: Vec<WipeData> = serde_json::from_str(&json)?;
            json_file.push(wipe_data.clone());
            for i in 0..json.len() {
                for j in i + 1..json.len() {
                    //iとjを比較してエリアネームが同じだったら削除
                    if let Some(e) = json_file.get(j) {
                        if json_file.get(i).unwrap().area_name.eq(e.area_name.as_str()) {
                            json_file.get_mut(i).unwrap().wipe_count = wipe_data.wipe_count;
                            json_file.remove(j);
                        };
                    } else {
                        break;
                    }
                }
            }
            println!("{:?}", json_file);
            let serialized: String = if json_file.is_empty() {
                json
            } else {
                let serialized = serde_json::to_string(&json_file)?;
                serialized
            };
            let mut write = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&file_name)?;
            write.write_all(serialized.as_bytes())?;
        } else {
            //ファイルがない場合
            let array = [wipe_data];
            let serialized = serde_json::to_string(&array)?;
            let mut file = fs::File::create(&file_name)?;
            file.write_all(serialized.as_bytes())?;
        }

        return Ok(());
    }

    fn is_file(file_name: &str) -> bool {
        return Path::new(file_name).exists() && fs::metadata(file_name).unwrap().is_file();
    }

    fn open_json(file_name: &str) -> anyhow::Result<Option<String>> {
        if !FileHandler::is_file(&file_name) {
            return Ok(None);
        };
        let mut file = fs::OpenOptions::new().read(true).open(&file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return Ok(Some(contents));
    }

    pub fn area_list(file_name: &str) -> anyhow::Result<Option<Vec<WipeData>>> {
        let read_file = FileHandler::open_json(&file_name)?;
        let Some(file_contents) = read_file else {
            return Ok(None);
        };
        let json: Vec<WipeData> = serde_json::from_str(&file_contents)?;
        return Ok(Some(json));
    }
}
