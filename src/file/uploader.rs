use std::process::Command;

pub struct Uploader;

impl Uploader {
    fn get_user(&self) -> anyhow::Result<String> {
        let output = Command::new("cmd")
            .args(&["/C", "echo %username%"])
            .output()?;

        let username = String::from_utf8(output.stdout)?.trim().to_string();
        println!("Username: {}", username);
        return Ok(username);
    }

    pub fn open_uploader(&self) -> anyhow::Result<()> {
        let username = self.get_user()?;
        let path = format!(
            r"C:\Users\{}\AppData\Local\Programs\FF Logs Uploader\FF Logs Uploader.exe",
            username
        );
        println!("{}", path);
        let _ = Command::new(path).output()?;
        return Ok(());
    }
}
