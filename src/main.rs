mod datetime;
mod fflogs;
mod file;
mod request;

use crate::{
    file::wipe_graph::WipeGraph,
    request::{msg_handler::MsgHandler, res_json::Phases},
};
use fflogs::fflogs::FFlogs;
use file::file_handler::FileHandler;
use request::{post_api::last_fights, res_json::JsonBool};
use tokio::io::{self, AsyncBufReadExt};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //色々と保存するファイル名
    const FILE_NAME: &str = "konoyonoowari.json";
    const WIPE_NAME: &str = "wipe_count.json";
    //ここからはlogsのapiキーを取得する
    let token = request::logs::Logs::get_token(&FILE_NAME).await?;
    //ここからdiscordのwebhookキーを取得する
    let hook_url = FileHandler::web_hook(&FILE_NAME, token).await?;
    //アップローダー起動してみるか
    let uploader = file::uploader::Uploader;
    if let Err(_) = uploader.open_uploader() {
        println!("fflogsuploaderを起動できませんでした");
    }
    //ここからlog取得やメッセージ送信
    const TIME_OUT: u64 = 1000;
    const LOOP_TIME: u64 = 100000;
    let id = FFlogs::url_input()?;
    let mut fight: Option<u64> = None;
    let mut wipe_count: u64 = 0;
    let mut last_area = String::new();
    let mut wipe_data: Vec<u8> = Vec::new();
    let mut time: i64 = 0;
    const DELAY: i64 = 3;
    let hook_rc = std::sync::Arc::new(hook_url.webhook.clone());
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
    println!("実行中 endで終了できます。");

    //別スレッドで入力を受け付ける
    tokio::task::spawn(async move {
        let mut reader = io::BufReader::new(io::stdin());
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();
            if !line.is_empty() {
                tx.send(line).await.unwrap();
            }
        }
    });
    //ガチで色々どうにかしたい
    loop {
        let now_time = chrono::Local::now().timestamp();
        let phases = Phases {
            id: String::from("id"),
            phases: 0,
        };
        let msg_handler = MsgHandler::new(hook_url.clone(), id.clone(), wipe_count);
        let last_fight = last_fights(
            &msg_handler.get_id(),
            &msg_handler.get_hook().key,
            request::post_api::Type::WIPE,
        )
        .await?;
        //戦闘エリアが取得nullの場合はスルーする。
        if time + DELAY < now_time {
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
                                let lf = last_fights(
                                    &msg_handler.get_id(),
                                    &msg_handler.get_hook().key,
                                    request::post_api::Type::KILL,
                                )
                                .await?;
                             
                                lf.send_msg(&lf.get_rankings(), &hook_url.webhook).await?;
                            }
                        }
                        //倒して初回起動のときの動作(ほぼテスト)
                        None => {
                            wipe_count += 1;
                            /*
                            wipe_count = msg_handler
                                .kill_msg(&area_name, wipe_count, &last_fight, &mut fight)
                                .await?;
                            let lf = last_fights(
                                &msg_handler.get_id(),
                                &msg_handler.get_hook().key,
                                request::post_api::Type::KILL,
                            )
                            .await?;
                            lf.send_msg(&lf.get_rankings(), &hook_url.webhook).await?;
                            */
                        }
                    },
                    //ワイプしたときの処理
                    JsonBool::FALSE => match fight {
                        Some(v) => {
                            //この条件分岐がtrueにならない場合は何も更新が起こっていないので何も起こらない
                            if last_fight.get_id().unwrap() > v {
                                //ワイプしてログが更新された時の動作
                                //なんとなく値渡しして上書きしたほうがわかりやすい気がした。
                                wipe_count += 1;
                                if last_area.is_empty() {
                                    //初回
                                    println!("初回");
                                    last_area = area_name.clone();
                                } else if last_area.ne(&area_name) {
                                    //エリアが違う場合リセット
                                    println!("reset");
                                    wipe_count = 1;
                                    last_area = area_name.clone();
                                }

                                let _ = msg_handler
                                    .wipe_msg(&area_name, wipe_count, &last_fight, &mut fight)
                                    .await?;

                                let mut wd = file::wipe_data::WipeData {
                                    area_name: area_name.clone(),
                                    wipe_count: wipe_count,
                                };
                                //ワイプカウントファイル書き出し
                                if let Some(areas) = FileHandler::area_list(&WIPE_NAME)? {
                                    for area in areas {
                                        if area.area_name.eq(&area_name) {
                                            //同じエリアが存在したらワイプ回数のみ増やす
                                            wd.area_name = area.area_name.clone();
                                            wd.wipe_count = area.wipe_count + 1;
                                            break;
                                        }
                                    }
                                }
                                //初回起動
                                else {
                                    println!("初回起動？");
                                    wd.area_name = area_name.clone();
                                    wd.wipe_count = wipe_count;
                                }
                                wipe_data.push(
                                    last_fight.get_phases().clone().unwrap_or(phases).phases as u8,
                                );
                                let _ = FileHandler::wipe_count(&wd)?;
                            }
                        }
                        None => {
                            //初回起動のときの動作(ほぼテスト)
                            //なんとなく値渡しして上書きしたほうがわかりやすい気がした。
                            wipe_count += 1;
                            last_area = area_name.clone();
                            /* 
                                //初回
                                println!("初回");
                                last_area = area_name.clone();
                            }
                            let _ = msg_handler
                                .wipe_msg(&area_name, wipe_count, &last_fight, &mut fight)
                                .await?;
                            let mut wd = file::wipe_data::WipeData {
                                area_name: String::new(),
                                wipe_count: 0,
                            };
                            //ワイプカウントファイル書き出し
                            if let Some(areas) = FileHandler::area_list(&WIPE_NAME)? {
                                for area in areas {
                                    if area.area_name.eq(&area_name) {
                                        wd.area_name = area.area_name.clone();
                                        wd.wipe_count = area.wipe_count + 1;
                                        break;
                                    }
                                }
                            }
                            //初回起動
                            else {
                                wd.area_name = area_name.clone();
                                wd.wipe_count = wipe_count;
                            }
                            let _ = FileHandler::wipe_count(&wd)?;
                            wipe_data.push(
                                last_fight.get_phases().clone().unwrap_or(phases).phases as u8,
                            );
                            */
                        }
                    },
                    JsonBool::NULL => (),
                }
            }
            time = now_time;
        }
        //終了するときの動作
        match tokio::time::timeout(tokio::time::Duration::from_micros(TIME_OUT), rx.recv()).await {
            Ok(Some(message)) => {
                //めちゃくちゃすぎるからいずれどうにかしたい
                if message.trim() == "end" {
                    let (mw, mp) = match WipeGraph::new().create_graph(&wipe_data, &last_area) {
                        Ok((image, mw, mp)) => {
                            let _ = last_fight
                                //rcの意味なくね なんでrc使ったのか思い出せない。
                                .send_file(hook_rc.clone(), &image)
                                .await?;
                            (Some(mw), Some(mp))
                        }
                        _ => (None, None),
                    };
                    let msg: String = match mw {
                        Some(phases) => {
                            format!("本日のワイプ数は{}回で一番ワイプしたフェーズはP{}の{}回でした。活動お疲れ様でした。", wipe_count,mp.unwrap(),phases)
                        }
                        None => {
                            format!(
                                "本日のワイプ数は{}回でした。活動お疲れ様でした。",
                                wipe_count
                            )
                        }
                    };
                    let datetime = datetime::DateTime::get_dt();
                    let end_msg = format!("-----------{}-----------", &datetime);
                    last_fight.send_msg(&msg, &hook_url.webhook).await?;
                    last_fight.send_msg(&end_msg, &hook_url.webhook).await?;
                    println!("enterを押して閉じる...");
                    return Ok(());
                }
            }
            _ => (),
        }
        let _ = tokio::time::sleep(tokio::time::Duration::from_micros(LOOP_TIME)).await;
    }
}
