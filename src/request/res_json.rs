use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResJson {
    data:Data,
}

impl ResJson {
    pub fn get_figths (&self) -> &Vec<Figth> {
        self.data.reportData.report.fights.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    reportData:ReportData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReportData {
    report:Report,
}
#[derive(Serialize, Deserialize, Debug)]
struct Report {
    fights:Vec<Figth>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Figth {
    id:u64,
}

impl Figth {
    pub fn get_id (&self) -> u64 {
        self.id
    }
}