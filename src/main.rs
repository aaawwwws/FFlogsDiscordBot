mod fflogs;
mod file;
mod request;
use fflogs::fflogs::FFlogs;
use file::file_handler::FileHandler;
use request::post_api::last_fight;
use tokio::time;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //色々と保存するファイル名
    let file_name: String = String::from("konoyonoowari.json");
    //ここからはlogsのapiキーを取得する
    let token = request::logs::Logs::get_token(&file_name).await?;
    //ここからdiscordのwebhookキーを取得する
    let hook_url = FileHandler::web_hook(&file_name, token).await?;
    //アップローダー起動してみるか
    let uploader = file::uploader::Uploader;
    if let Err(_) = uploader.open_uploader() {
        println!("fflogsuploaderを起動できませんでした");
    }
    //ここからlog取得やメッセージ送信reportId
    let id = FFlogs::url_input()?;
    let mut fight: Option<u64> = None;
    println!("実行中");
    loop {
        match fight {
            Some(v) => {
                let last_fight = last_fight(&id, &hook_url.key).await?;
                if last_fight.get_id().unwrap() > v {
                    last_fight
                        .new_msg(&id, last_fight.get_id().unwrap(), &hook_url.webhook)
                        .await?;
                    fight = Some(last_fight.get_id().unwrap());
                }
            }
            None => {
                let last_fight = last_fight(&id, &hook_url.key).await?;
                fight = Some(last_fight.get_id().unwrap());
                last_fight
                    .new_msg(&id, last_fight.get_id().unwrap(), &hook_url.webhook)
                    .await?;
            }
        }
        let _ = time::sleep(time::Duration::from_secs(1)).await;
    }
}
