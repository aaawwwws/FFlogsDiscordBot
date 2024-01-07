use crate::file::file_handler::FileHandler;
pub struct FFlogs;
impl FFlogs {
    pub fn url_input() -> anyhow::Result<String> {
        let mut url_split: Vec<String> = Vec::new();
        loop {
            url_split.clear();
            let url = FileHandler::input("レポートのURLを入力してください\n例:https://ja.fflogs.com/reports/aaaaaaaaaaaaaaaaaaaaaaaaaa")?;
            //URLが正しいかの確認
            let a: Vec<&str> = url.split("/").collect();
            a.iter().for_each(|e| {
                url_split.push(e.to_string());
            });
            if url_split.len() > 5
                || !url_split[0].eq("https:")
                || !url_split[2].contains("fflogs.com")
                || !url_split[3].eq("reports")
            {
                println!("URLを確認してください")
            } else {
                break;
            }
        }
        if url_split.last().unwrap().contains("#") {
            //report以降のコピーを取る
           let last = url_split.last().unwrap().clone();
           //コピー取ったやつを"#"で分ける。
           let split: Vec<&str> = last.split("#").collect();
           //元のデータの最後を削除
           url_split.pop();
           //"#"の左辺を追加
           url_split.push(split.first().unwrap().to_string())
        }
        //reportidのみ取り出す
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
