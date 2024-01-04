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
    //アップローダー起動してみるか
    let uploader = file::uploader::Uploader;
    if let Err(_) = uploader.open_uploader() {
        println!("fflogsuploaderを起動できませんでした");
    }
    //ここからはlogsのapiキーを取得する
    let token = request::logs::Logs::get_token(&file_name).await?;
    //ここからdiscordのwebhookキーを取得する
    let hook_url = FileHandler::web_hook(&file_name, token)?;
    //ここからlog取得やメッセージ送信reportId
    let id = FFlogs::url_input()?;
    let mut figth: Option<u64> = None;
    println!("実行中");
    loop {
        match figth {
            Some(v) => {
                let last_fight = last_fight(&id, &hook_url.key).await?;
                if last_fight.get_id() > v {
                    last_fight
                        .send_msg(&id, last_fight.get_id(), &hook_url.webhook)
                        .await?;
                    figth = Some(last_fight.get_id());
                }
            }
            None => {
                let last_fight = last_fight(&id, &hook_url.key).await?;
                figth = Some(last_fight.get_id());
                last_fight
                    .send_msg(&id, last_fight.get_id(), &hook_url.webhook)
                    .await?;
            }
        }
        let _ = time::sleep(time::Duration::from_secs(1)).await;
    }
}
