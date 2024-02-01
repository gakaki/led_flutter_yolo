use chrono::{DateTime, Local};

pub fn current_time() -> String {
    // 获取本地时区的当前时间
    let now: DateTime<Local> = Local::now();
    // 格式化时间为 yyyy-mm-dd hh:mm:ss
    let formatted_now = now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("当前时间: {}", formatted_now);
    formatted_now.clone()
}
