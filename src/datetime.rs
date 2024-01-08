use chrono::Local;

pub struct DateTime;

impl DateTime {
    pub fn get_dt() -> String {
        let dt = Local::now();
        let time_str = dt.format("%Y-%m-%d %H:%M:%S");
        time_str.to_string()
    }

    pub fn get_time() -> String {
        let dt = Local::now();
        let time_str = dt.format("%H:%M:%S");
        time_str.to_string()
    }
}
