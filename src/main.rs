mod request;
use request::post_api::last_fight;
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let id = String::from("drPy8Mzf1WmY7jkb");
    let mut figth: Option<u64> = None;
    let mut num:u64 = 0;
    loop {
        num += 1;
        match figth {
            Some(v)  => {
                let last_fight = last_fight(&id).await?;
                if last_fight > v {
                    println!("更新されました。{}回目の取得 URL:https://ja.fflogs.com/reports/{}#fight={}",&num, id, last_fight);
                    figth = Some(last_fight);
                }
                let _ = time::sleep(time::Duration::from_secs(2)).await;
            }
            None => {
                let last_fight = last_fight(&id).await?;
                println!("{}回目の取得 URL:https://ja.fflogs.com/reports/{}#fight={}",&num, id, last_fight);
                figth = Some(last_fight);
                let _ = time::sleep(time::Duration::from_secs(2)).await;
            }
        }
    }
}
