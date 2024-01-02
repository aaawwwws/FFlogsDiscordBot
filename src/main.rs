mod request;
use request::{post_api::last_fight, post_discord};
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //ここからlogs取得やメッセージ送信
    let id = String::from("fPm8K32vRaNXxyWd");
    let mut figth: Option<u64> = None;
    loop {
        match figth {
            Some(v) => {
                let last_fight = last_fight(&id).await?;
                if last_fight.get_id() > v {
                    last_fight.send_msg(&id, last_fight.get_id()).await?;
                    figth = Some(last_fight.get_id());
                }
                let _ = time::sleep(time::Duration::from_secs(2)).await;
            }
            None => {
                let last_fight = last_fight(&id).await?;
                figth = Some(last_fight.get_id());
                last_fight.send_msg(&id, last_fight.get_id()).await?;
                let _ = time::sleep(time::Duration::from_secs(2)).await;
            }
        }
    }
}
