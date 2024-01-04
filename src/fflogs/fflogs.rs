use crate::file::file_handler::FileHandler;
use anyhow::anyhow;
pub struct FFlogs;
impl FFlogs {
    pub fn url_input() -> anyhow::Result<String> {
        let url = FileHandler::input("レポートのURLを入力してください\n例:https://ja.fflogs.com/reports/aaaaaaaaaaaaaaaaaaaaaaaaaa")?;
        let mut url_split: Vec<&str> = url.split("/").collect();
        if url_split.len() > 5
            || !url_split[0].eq("https:")
            || !url_split[2].contains("fflogs.com")
            || !url_split[3].eq("reports")
        {
            return Err(anyhow!("URLを確認してください。"));
        }
        //URLに"#"が含まれていた場合
        if url_split.last().unwrap().contains("#") {
            let param: Vec<&str> = url_split.last().unwrap().split("#").collect();
            let reportid = param.first().unwrap();
            url_split.remove(url_split.len() - 1);
            url_split.push(reportid);
        }

        let report_id: String = url_split
            .last()
            .unwrap()
            .trim_end_matches("\r")
            .trim_end_matches("\n")
            .trim_end_matches("\r")
            .to_string();
        return Ok(report_id);
    }
}
