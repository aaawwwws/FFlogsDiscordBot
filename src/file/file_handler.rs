use std::{fs, io, io::Write, path::Path};

pub struct FileHandler;

impl FileHandler {
    pub fn input(msg: &str) -> anyhow::Result<String> {
        println!("{}", msg);
        let mut cin = String::new();
        let _ = io::stdin().read_line(&mut cin)?;
        Ok(cin)
    }
    pub fn web_hook() -> anyhow::Result<String> {
        let file_name: String = String::from("konoyonoowari.txt");
        let file_check =
            Path::new(&file_name).exists() && fs::metadata(&file_name).unwrap().is_file();
        if file_check {
            //ファイルが存在する場合 or 2回目以降の処理
            let f = fs::read_to_string(&file_name)?;
            return Ok(f);
        } else {
            //初回起動 or ファイルが存在しない場合
            let mut hook_url = FileHandler::input("webhookのURLを入力してください")?;
            let mut file = fs::File::create(&file_name)?;
            write!(&mut file, "{}", hook_url)?;
            let f = fs::read_to_string(&file_name)?;
            return Ok(f);
        }
    }
}
