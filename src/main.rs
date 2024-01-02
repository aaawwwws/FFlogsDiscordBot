mod fflogs;
mod file;
mod request;
use fflogs::fflogs::FFlogs;
use file::file_handler::FileHandler;
use request::post_api::last_fight;
use tokio::time;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //ここからdiscordのwebhookキーを取得する
    let hook_url = FileHandler::web_hook()?;
    //ここからlog取得やメッセージ送信
    let id = FFlogs::url_input()?;
    let mut figth: Option<u64> = None;
    println!("{}", &id);
    loop {
        match figth {
            Some(v) => {
                let last_fight = last_fight(&id).await?;
                if last_fight.get_id() > v {
                    last_fight
                        .send_msg(&id, last_fight.get_id(), &hook_url)
                        .await?;
                    figth = Some(last_fight.get_id());
                }
                let _ = time::sleep(time::Duration::from_secs(2)).await;
            }
            None => {
                let last_fight = last_fight(&id).await?;
                figth = Some(last_fight.get_id());
                last_fight
                    .send_msg(&id, last_fight.get_id(), &hook_url)
                    .await?;
                let _ = time::sleep(time::Duration::from_secs(2)).await;
            }
        }
    }
}
