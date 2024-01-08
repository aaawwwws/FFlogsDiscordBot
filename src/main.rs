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
    let mut wipe_count: u64 = 0;
    println!("実行中");
    loop {
        let msg_handler = MsgHandler::new(hook_url.clone(), id.clone(), wipe_count);
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
                            wipe_count = msg_handler
                                .kill_msg(&area_name, wipe_count, &last_fight, &mut fight)
                                .await?;
                        }
                    }
                    //倒して初回起動のときの動作(ほぼテスト)
                    None => {
                        wipe_count = msg_handler
                            .kill_msg(&area_name, wipe_count, &last_fight, &mut fight)
                            .await?;
                    }
                },
                //ワイプしたときの処理
                JsonBool::FALSE => match fight {
                    Some(v) => {
                        //この条件分岐がtrueにならない場合は何も更新が起こっていないので何も起こらない
                        if last_fight.get_id().unwrap() > v {
                            //ワイプしてログが更新された時の動作
                            //なんとなく値渡しして上書きしたほうがわかりやすいきがした。
                            wipe_count = msg_handler
                                .wipe_msg(&area_name, wipe_count, &last_fight, &mut fight)
                                .await?;
                        }
                    }
                    None => {
                        //初回起動のときの動作(ほぼテスト)
                        //なんとなく値渡しして上書きしたほうがわかりやすいきがした。
                        wipe_count = msg_handler
                            .wipe_msg(&area_name, wipe_count, &last_fight, &mut fight)
                            .await?;
                    }
                },
                JsonBool::NULL => (),
            }
        }
        let _ = time::sleep(time::Duration::from_secs(1)).await;
    }
}
