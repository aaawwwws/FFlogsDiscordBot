use std::clone;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResJson {
    data: Data,
}

impl ResJson {
    pub fn get_figths(&self) -> Option<&Vec<Figth>> {
        if let Some(f) = &self.data.reportData.report.fights {
            return Some(f.as_ref());
        } else {
            return None;
        }
    }

    pub fn get_rankig_role(&self) -> Option<&Roles> {
        if let Some(rankings) = &self.data.reportData.report.rankings.as_ref() {
            return Some(&rankings.data.last().unwrap().roles);
        } else {
            return None;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Data {
    reportData: ReportData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReportData {
    report: Report,
}
#[derive(Serialize, Deserialize, Debug)]
struct Report {
    fights: Option<Vec<Figth>>,
    rankings: Option<DData>,
}
#[derive(Serialize, Deserialize, Debug)]
struct DData {
    data: Vec<RankingsData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct RankingsData {
    fightID: u64,
    partition: u64,
    zone: u64,
    encounter: Encounter,
    difficulty: u64,
    size: u64,
    kill: u64,
    duration: u64,
    bracketData: f64,
    deaths: u64,
    damageTakenExcludingTanks: u64,
    roles: Roles,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Roles {
    tanks: Tank,
    healers: Healer,
    dps: Dps,
}

impl Roles {
    pub fn get_tanks(&self) -> &Tank {
        &self.tanks
    }

    pub fn get_healers(&self) -> &Healer {
        &self.healers
    }

    pub fn get_dps(&self) -> &Dps {
        &self.dps
    }
}

pub trait Charas {
    fn get_characters(&self) -> &Vec<Character>;
}
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Tank {
    name: String,
    characters: Vec<Character>,
}

impl Charas for Tank {
    fn get_characters(&self) -> &Vec<Character> {
        &self.characters
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Healer {
    name: String,
    characters: Vec<Character>,
}

impl Charas for Healer {
    fn get_characters(&self) -> &Vec<Character> {
        &self.characters
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dps {
    name: String,
    characters: Vec<Character>,
}

impl Charas for Dps {
    fn get_characters(&self) -> &Vec<Character> {
        &self.characters
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Character {
    id: u64,
    name: String,
    server: Option<Server>,
    class: String,
    spec: String,
    amount: f64,
    bracketData: f32,
    bracket: u64,
    rank: String,
    best: String,
    totalParses: u64,
    bracketPercent: u64,
    rankPercent: u64,
}

impl Character {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_class(&self) -> &str {
        &self.class
    }

    pub fn get_amount(&self) -> &f64 {
        &self.amount
    }

    pub fn get_rank_per(&self) -> &u64 {
        &self.rankPercent
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Server {
    id: u64,
    name: String,
    region: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Encounter {
    id: u64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Figth {
    id: u64,
    kill: serde_json::Value,
    name: serde_json::Value,
    phaseTransitions: serde_json::Value,
}

impl Figth {
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn get_killtype(&self) -> JsonBool {
        match self.kill {
            serde_json::Value::Bool(v) => {
                if v {
                    return JsonBool::TRUE;
                }
                return JsonBool::FALSE;
            }
            serde_json::Value::Null => return JsonBool::NULL,
            _ => return JsonBool::NULL,
        }
    }
    pub fn get_name(&self) -> Option<String> {
        match &self.name {
            serde_json::Value::String(value) => return Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_phases(&self) -> anyhow::Result<Option<Phases>> {
        match &self.phaseTransitions {
            serde_json::Value::Array(phases) => {
                let phases_map = phases.last().unwrap().as_object().unwrap();
                if let serde_json::Value::Number(num) = phases_map.get("id").unwrap() {
                    let p = Phases {
                        id: String::from("id"),
                        phases: num.as_i64().unwrap(),
                    };
                    return Ok(Some(p));
                }
                return Ok(None);
            }
            _ => Ok(None),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    pub access_token: String,
    expires_in: i64,
    token_type: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Konoyonoowari {
    pub key: String,
    pub webhook: String,
}
#[derive(Debug)]
pub enum JsonBool {
    TRUE,
    FALSE,
    NULL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Phases {
    pub id: String,
    pub phases: i64,
}
