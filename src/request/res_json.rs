use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResJson {
    data: Data,
}

impl ResJson {
    pub fn get_figths(&self) -> &Vec<Figth> {
        self.data.reportData.report.fights.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    reportData: ReportData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReportData {
    report: Report,
}
#[derive(Serialize, Deserialize, Debug)]
struct Report {
    fights: Vec<Figth>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Figth {
    id: u64,
    kill: serde_json::Value,
    name: serde_json::Value,
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
