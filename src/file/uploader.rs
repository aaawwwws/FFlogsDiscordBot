use console::Term;
use std::process::{Command, Stdio};
pub struct Uploader;

impl Uploader {
    fn get_user(&self) -> anyhow::Result<String> {
        let output = Command::new("cmd")
            .args(&["/C", "echo %username%"])
            .output()?;

        let username = String::from_utf8(output.stdout)?.trim().to_string();
        return Ok(username);
    }

    pub fn open_uploader(&self) -> anyhow::Result<()> {
        loop {
            let input = crate::file::file_handler::FileHandler::input(
                "fflogsuploaderを起動しますか？ y/n",
            )?;
            match input.as_str() {
                "y" => break,
                "n" => return Ok(()),
                _ => (),
            }
        }
        let username = self.get_user()?;
        let path = format!(
            r"C:\Users\{}\AppData\Local\Programs\FF Logs Uploader\FF Logs Uploader.exe",
            username
        );
        let _ = Command::new(path)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Term::stdout().clear_screen()?;
        return Ok(());
    }
}
