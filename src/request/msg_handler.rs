use crate::datetime;

use super::{post_api::last_fight, post_discord::PostDiscord, res_json::Konoyonoowari};

pub struct MsgHandler {
    _hook: Konoyonoowari,
    _id: String,
    _fight: Option<u64>,
    _wipe_count: u64,
}

impl MsgHandler {
    pub fn new(hook: Konoyonoowari, id: String, fight: Option<u64>, wipe_count: u64) -> Self {
        return Self {
            _hook: hook,
            _id: id,
            _fight: fight,
            _wipe_count: wipe_count,
        };
    }

    pub async fn wipe_msg(
        &self,
        area_name: &str,
        wipe_count: &u64,
        last_fight: PostDiscord,
        fight: &mut Option<u64>,
    ) -> anyhow::Result<()> {
        let datetime = datetime::DateTime::get_time();
        let url = format!(
            "https://ja.fflogs.com/reports/{}#fight={}",
            &self._id,
            self._fight.unwrap()
        );
        //初回時のみtrue
        if let None = fight {
            let first_msg = format!("-----------{}-----------", &datetime);
            let _ = last_fight.send_msg(&first_msg, &self._hook.webhook).await?;
            *fight = Some(last_fight.get_id().unwrap());
        }
        let msg = format!(
            "時刻:{} エリア:{} {}ワイプ目:{}",
            datetime, area_name, *wipe_count, url
        );
        let _ = last_fight.send_msg(&msg, &self._hook.webhook).await?;
        return Ok(());
    }

    pub fn get_hook(&self) -> &Konoyonoowari {
        &self._hook
    }
    pub fn get_id(&self) -> &str {
        &self._id
    }
}
