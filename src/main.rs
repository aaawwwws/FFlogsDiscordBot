mod datetime;
mod fflogs;
mod file;
mod request;
use fflogs::fflogs::FFlogs;
use file::file_handler::FileHandler;
use request::{post_api::last_fight, res_json::JsonBool};
use tokio::time;

use crate::request::msg_handler::MsgHandler;

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
    let mut wipe_count: u64 = 1;
    let msg_handler = MsgHandler::new(hook_url, id, fight, wipe_count);
    println!("実行中");
    loop {
        let datetime = datetime::DateTime::get_time();
        let last_fight = last_fight(&msg_handler.get_id(), &msg_handler.get_hook().key).await?;
        //戦闘エリアが取得nullの場合はスルーする。
        if let Some(area_name) = last_fight.get_name() {
            match *last_fight.get_killtype() {
                //倒したときの処理
                JsonBool::TRUE => match fight {
                    Some(v) => {
                        //この条件分岐がtrueにならない場合は何も更新が起こっていないので何も起こらない
                        if last_fight.get_id().unwrap() > v {
                            //倒してログが更新されたときの動作
                            wipe_count = 1;
                            let url = format!(
                                "https://ja.fflogs.com/reports/{}#fight={}",
                                &msg_handler.get_id(),
                                last_fight.get_id().unwrap()
                            );
                            let msg = format!("倒しました:{} 場所{}", url, &area_name);
                            last_fight
                                .send_msg(
                                    &format!("-----------{}-----------", &datetime),
                                    &msg_handler.get_hook().webhook,
                                )
                                .await?;
                            let _ = last_fight
                                .send_msg(&msg, &msg_handler.get_hook().webhook)
                                .await?;
                        }
                    }
                    //倒して初回起動のときの動作
                    None => {
                        fight = Some(last_fight.get_id().unwrap());
                        let url = format!(
                            "https://ja.fflogs.com/reports/{}#fight={}",
                            &msg_handler.get_id(),
                            last_fight.get_id().unwrap()
                        );
                        let msg = format!("倒しました:{} 場所{}", url, &area_name);
                        last_fight
                            .send_msg(
                                &format!("-----------{}-----------", &datetime),
                                &msg_handler.get_hook().webhook,
                            )
                            .await?;
                        let _ = last_fight
                            .send_msg(&msg, &msg_handler.get_hook().webhook)
                            .await?;
                    }
                },
                //ワイプしたときの処理
                JsonBool::FALSE => match fight {
                    Some(v) => {
                        //この条件分岐がtrueにならない場合は何も更新が起こっていないので何も起こらない
                        if last_fight.get_id().unwrap() > v {
                            //ワイプしてログが更新された時の動作
                            last_fight
                                .new_msg(
                                    &msg_handler.get_id(),
                                    &mut wipe_count,
                                    &datetime,
                                    last_fight.get_id().unwrap(),
                                    &area_name,
                                    &msg_handler.get_hook().webhook,
                                )
                                .await?;
                            file::file_handler::FileHandler::wipe_count(wipe_count)?;
                            fight = Some(last_fight.get_id().unwrap());
                        }
                    }
                    None => {
                        //初回起動のときの動作
                        fight = Some(last_fight.get_id().unwrap());
                        last_fight
                            .send_msg(
                                &format!("-----------{}-----------", &datetime),
                                &msg_handler.get_hook().webhook,
                            )
                            .await?;
                        last_fight
                            .new_msg(
                                msg_handler.get_id(),
                                &mut wipe_count,
                                &datetime,
                                last_fight.get_id().unwrap(),
                                &area_name,
                                &msg_handler.get_hook().webhook,
                            )
                            .await?;
                    }
                },
                JsonBool::NULL => (),
            }
        }
        let _ = time::sleep(time::Duration::from_secs(1)).await;
    }
}
