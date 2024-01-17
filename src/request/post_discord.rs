use std::{
    collections::{hash_map, HashMap},
    sync::Arc,
};

use reqwest::{header, Client};

use crate::file::ranking_data::RankingData;

use super::res_json::{self, Charas, Phases, Roles};
#[derive(Debug)]
pub struct PostDiscord {
    _client: Client,
    _id: Option<u64>,
    _killtype: res_json::JsonBool,
    _name: Option<String>,
    _phases: Option<Phases>,
    _roles: Option<Roles>,
}

impl PostDiscord {
    pub fn new(
        client: Client,
        id: Option<u64>,
        killtype: res_json::JsonBool,
        name: Option<String>,
        phases: Option<Phases>,
        roles: Option<Roles>,
    ) -> Self {
        return Self {
            _client: client,
            _id: id,
            _killtype: killtype,
            _name: name,
            _phases: phases,
            _roles: roles,
        };
    }

    pub async fn send_msg(&self, content: &str, hook_url: &str) -> anyhow::Result<u16> {
        let query = format!(
            r#"{{
            "content": "{}"
            }}"#,
            content
        );

        let Ok(res) = self
            ._client
            .post(hook_url)
            .header("Content-Type", "application/json")
            .body(query)
            .send()
            .await
        else {
            return Err(anyhow::anyhow!("接続できませんでした。"));
        };
        return Ok(res.status().as_u16());
    }

    pub async fn send_file(&self, hook_url: Arc<String>, file_name: &str) -> anyhow::Result<u16> {
        let file = reqwest::blocking::multipart::Form::new().file("file", &file_name)?;
        let _ = tokio::task::spawn_blocking(move || {
            let _res = reqwest::blocking::Client::new()
                .post(hook_url.as_str())
                .multipart(file)
                .send();
        })
        .await?;

        return Ok(22);
    }

    pub fn get_id(&self) -> Option<u64> {
        return self._id;
    }
    pub fn get_killtype(&self) -> &res_json::JsonBool {
        return &self._killtype;
    }
    pub fn get_name(&self) -> Option<String> {
        return self._name.clone();
    }
    pub fn get_phases(&self) -> &Option<Phases> {
        return &self._phases;
    }
    fn crate_rankings(&self, role: Role) -> Vec<RankingData> {
        match role {
            Role::TANK => {
                return self
                    ._roles
                    .as_ref()
                    .unwrap()
                    .get_tanks()
                    .get_characters()
                    .iter()
                    .clone()
                    .map(|t| {
                        return RankingData {
                            name: t.get_name().to_string(),
                            amount: t.get_amount().to_string(),
                            class: t.get_class().to_string(),
                            rank_per: *t.get_rank_per(),
                        };
                    })
                    .collect();
            }
            Role::HEALER => {
                return self
                    ._roles
                    .as_ref()
                    .unwrap()
                    .get_healers()
                    .get_characters()
                    .iter()
                    .clone()
                    .map(|t| {
                        return RankingData {
                            name: t.get_name().to_string(),
                            amount: t.get_amount().to_string(),
                            class: t.get_class().to_string(),
                            rank_per: *t.get_rank_per(),
                        };
                    })
                    .collect();
            }
            Role::DPS => {
                return self
                    ._roles
                    .as_ref()
                    .unwrap()
                    .get_dps()
                    .get_characters()
                    .iter()
                    .clone()
                    .map(|d| {
                        return RankingData {
                            name: d.get_name().to_string(),
                            amount: d.get_amount().to_string(),
                            class: d.get_class().to_string(),
                            rank_per: *d.get_rank_per(),
                        };
                    })
                    .collect();
            }
        };
    }
    pub fn get_rankings(&self) -> String {
        let mut job_replace: HashMap<&str, &str> = HashMap::new();
        job_replace.insert("Astrologian", "占");
        job_replace.insert("Bard", "詩人");
        job_replace.insert("Black Mage", "黒");
        job_replace.insert("Dancer", "踊");
        job_replace.insert("DarkKnight", "暗");
        job_replace.insert("Dragoon", "竜");
        job_replace.insert("Gunbreaker", "ガ");
        job_replace.insert("Machinist", "機");
        job_replace.insert("Monk", "モ");
        job_replace.insert("Ninja", "忍");
        job_replace.insert("Paladin", "ナ");
        job_replace.insert("Reaper", "リ");
        job_replace.insert("RedMage", "赤");
        job_replace.insert("Sage", "賢");
        job_replace.insert("Samurai", "侍");
        job_replace.insert("Scholar", "学");
        job_replace.insert("Summoner", "召");
        job_replace.insert("Warrior", "戦");
        job_replace.insert("WhiteMage", "白");
        let mut tank = self.crate_rankings(Role::TANK);
        let mut healer = self.crate_rankings(Role::HEALER);
        let mut dps = self.crate_rankings(Role::DPS);
        self.cut(&mut tank, Role::TANK);
        self.cut(&mut healer, Role::HEALER);
        self.cut(&mut dps, Role::DPS);
        let total_rankings: Vec<RankingData> = Vec::new()
            .into_iter()
            .chain(tank)
            .chain(healer)
            .chain(dps)
            .collect();
        let result: Vec<String> = total_rankings
            .iter()
            .map(|r| {
                let mut job = String::new();
                for (k, v) in &job_replace {
                    if r.class.eq(k) {
                        job = r.class.replace(k, v);
                        break;
                    }
                }
                if r.rank_per == 100 {
                    return format!("{} perf:{}% 金\\n", job, r.rank_per);
                } else if r.rank_per == 99 {
                    return format!("{} perf:{}% 桃\\n", job, r.rank_per);
                } else if r.rank_per <= 98 && r.rank_per >= 95 {
                    return format!("{} perf:{}% 橙\\n", job, r.rank_per);
                } else if r.rank_per <= 94 && r.rank_per >= 75 {
                    return format!("{} perf:{}% 紫\\n", job, r.rank_per);
                } else if r.rank_per <= 74 && r.rank_per >= 50 {
                    return format!("{} perf:{}% 青\\n", job, r.rank_per);
                } else if r.rank_per <= 49 && r.rank_per >= 25 {
                    return format!("{} perf:{}% 緑\\n", job, r.rank_per);
                } else {
                    return format!("{} perf:{}% 灰\\n", job, r.rank_per);
                }
            })
            .collect();
        let mut str = String::new();
        result.iter().for_each(|s| str.push_str(s));
        str
    }

    fn cut(&self, roles: &mut Vec<RankingData>, role: Role) {
        if let Role::DPS = role {
            while roles.len() > 4 {
                roles.pop().unwrap();
                return;
            }
        } else {
            while roles.len() > 2 {
                roles.pop().unwrap();
                return;
            }
        }
    }
}

enum Role {
    TANK,
    HEALER,
    DPS,
}
