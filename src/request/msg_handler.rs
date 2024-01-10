use crate::datetime;

use super::{post_discord::PostDiscord, res_json::Konoyonoowari};

#[derive(Debug)]
pub struct MsgHandler {
    _hook: Konoyonoowari,
    _id: String,
    _wipe_count: u64,
}

impl MsgHandler {
    pub fn new(hook: Konoyonoowari, id: String, wipe_count: u64) -> Self {
        return Self {
            _hook: hook,
            _id: id,
            _wipe_count: wipe_count,
        };
    }

    pub async fn wipe_msg(
        &self,
        area_name: &str,
        wipe_count: u64,
        last_fight: &PostDiscord,
        fight: &mut Option<u64>,
    ) -> anyhow::Result<()> {
        let datetime = datetime::DateTime::get_dt();
        let time = datetime::DateTime::get_time();
        //初回時のみtrue
        if let None = fight {
            let first_msg = format!("-----------{}-----------", &datetime);
            let _ = last_fight.send_msg(&first_msg, &self._hook.webhook).await?;
        }
        *fight = Some(last_fight.get_id().unwrap());
        let url = format!(
            "https://ja.fflogs.com/reports/{}#fight={}",
            &self._id,
            fight.unwrap()
        );
        let msg = format!(
            "**wipe!**   時刻:{}   ボス(エリア):{}   ワイプ回数:{}   ログ:{}",
            time, area_name, wipe_count, url
        );
        let _ = last_fight.send_msg(&msg, &self._hook.webhook).await?;
        return Ok(());
    }

    pub async fn kill_msg(
        &self,
        area_name: &str,
        wipe_count: u64,
        last_fight: &PostDiscord,
        fight: &mut Option<u64>,
    ) -> anyhow::Result<u64> {
        let datetime = datetime::DateTime::get_dt();
        let time = datetime::DateTime::get_time();
        //初回時のみtrue
        if let None = fight {
            let first_msg = format!("-----------{}-----------", &datetime);
            let _ = last_fight.send_msg(&first_msg, &self._hook.webhook).await?;
        }
        *fight = Some(last_fight.get_id().unwrap());
        let url = format!(
            "https://ja.fflogs.com/reports/{}#fight={}",
            &self._id,
            fight.unwrap()
        );
        let msg = format!(
            "**kill!**   時刻:{}   ボス(エリア):{}   ワイプ回数:{}   ログ:{}   ",
            time, area_name, wipe_count, url
        );
        let count = 0;
        let _ = last_fight.send_msg(&msg, &self._hook.webhook).await?;
        return Ok(count);
    }

    pub fn get_hook(&self) -> &Konoyonoowari {
        &self._hook
    }
    pub fn get_id(&self) -> &str {
        &self._id
    }
}
