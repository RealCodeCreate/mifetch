use std::fs;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // 1. Получаем имя пользователя и хостнейм
    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let mut host_buf = [0u8; 64];
    let host_name = if let Ok(mut f) = fs::File::open("/proc/sys/kernel/hostname") {
        let n = f.read(&mut host_buf).unwrap_or(0);
        String::from_utf8_lossy(&host_buf[..n]).trim().to_string()
    } else {
        "unknown".to_string()
    };
    let host_line = format!("{}@{}", user, host_name);

    // 2. Получаем название ОС (из /etc/os-release)
    let mut os_name = "Linux".to_string();
    if let Ok(mut f) = fs::File::open("/etc/os-release") {
        let mut os_buf = [0u8; 1024];
        if let Ok(n) = f.read(&mut os_buf) {
            let content = String::from_utf8_lossy(&os_buf[..n]);
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    os_name = line.replace("PRETTY_NAME=", "").replace('"', "");
                    break;
                }
            }
        }
    }

    // 3. Uptime
    let uptime_str = fs::read_to_string("/proc/uptime").unwrap_or_default();
    let uptime_sec = uptime_str.split_whitespace().next().unwrap_or("0").parse::<f32>().unwrap_or(0.0) as u64;
    let h = uptime_sec / 3600;
    let m = (uptime_sec % 3600) / 60;
    let uptime = format!("{}h {}m", h, m);

    // 4. Ядро
    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_default().trim().to_string();

    // 5. RAM
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = 0;
    let mut available = 0;
    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).unwrap_or("0").parse::<u64>().unwrap_or(0) / 1024;
        }
        if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1).unwrap_or("0").parse::<u64>().unwrap_or(0) / 1024;
        }
    }
    let ram = format!("{}M / {}M", total - available, total);

    // Вывод
    println!(
        "┏━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓\n\
         ┃   .~.   ┃  {H:<27}┃\n\
         ┃   /V\\   ┃  os      {OS:<19}┃\n\
         ┃  // \\\\  ┃  uptime  {UP:<19}┃\n\
         ┃ /(   )\\ ┃  kernel  {K:<19}┃\n\
         ┃  ^`~'^  ┃  ram     {RAM:<19}┃\n\
         ┗━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
        H=host_line, OS=os_name, UP=uptime, K=kernel, RAM=ram
    );

    Ok(())
}
