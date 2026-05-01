use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // Используем заранее выделенный буфер на стеке для вывода
    let mut buffer = [0u8; 2048];
    let mut pos = 0;

    // Цвета
    let c1 = "\x1b[38;5;31m";
    let c2 = "\x1b[38;5;74m";
    let c3 = "\x1b[38;5;117m";
    let c4 = "\x1b[38;5;195m";
    let r = "\x1b[0m";

    // 1. Host (читаем напрямую без аллокаций String)
    let user = std::env::var("USER").unwrap_or_else(|_| "user".into());
    let mut hostname = [0u8; 64];
    let host_len = File::open("/proc/sys/kernel/hostname")
        .and_then(|mut f| f.read(&mut hostname))
        .unwrap_or(0);
    let host_str = std::str::from_utf8(&hostname[..host_len]).unwrap_or("").trim();
    let host_line = format!("{}@{}", user, host_str);

    // 2. OS (быстрый поиск в /etc/os-release)
    let mut os_buf = [0u8; 512];
    let mut os_name = "Linux";
    if let Ok(mut f) = File::open("/etc/os-release") {
        if let Ok(n) = f.read(&mut os_buf) {
            let content = String::from_utf8_lossy(&os_buf[..n]);
            for line in content.lines() {
                if line.starts_with("NAME=") {
                    os_name = line[5..].trim_matches('"');
                    break;
                }
            }
        }
    }

    // 3. Kernel (читаем /proc/sys/kernel/osrelease)
    let mut k_buf = [0u8; 128];
    let mut kernel = "unknown";
    if let Ok(mut f) = File::open("/proc/sys/kernel/osrelease") {
        if let Ok(n) = f.read(&mut k_buf) {
            kernel = std::str::from_utf8(&k_buf[..n]).unwrap_or("").trim();
        }
    }

    // 4. Uptime (парсим только первое число)
    let mut u_buf = [0u8; 64];
    let mut uptime = String::new();
    if let Ok(mut f) = File::open("/proc/uptime") {
        if let Ok(n) = f.read(&mut u_buf) {
            let s = std::str::from_utf8(&u_buf[..n]).unwrap_or("");
            if let Some(first) = s.split_whitespace().next() {
                if let Ok(secs_f) = first.parse::<f32>() {
                    let s = secs_f as u64;
                    uptime = format!("{}h {}m", s / 3600, (s % 3600) / 60);
                }
            }
        }
    }

    // 5. RAM (MemTotal и MemAvailable)
    let mut m_buf = [0u8; 1024];
    let mut ram = String::new();
    if let Ok(mut f) = File::open("/proc/meminfo") {
        if let Ok(n) = f.read(&mut m_buf) {
            let content = String::from_utf8_lossy(&m_buf[..n]);
            let mut total = 0;
            let mut avail = 0;
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    total = line.split_ascii_whitespace().nth(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                } else if line.starts_with("MemAvailable:") {
                    avail = line.split_ascii_whitespace().nth(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                }
                if total > 0 && avail > 0 { break; }
            }
            ram = format!("{}M / {}M", (total - avail) / 1024, total / 1024);
        }
    }

    // Формируем финальный вывод
    let output = format!(
"{C1}┏━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃{C2}   .{C4}~{C2}.   {C1}┃  {C4}{H:<27} {C1}┃
┃{C2}   {C2}/{C4}V{C2}\\   {C1}┃  {C3}os       {C4}{OS:<18} {C1}┃
┃{C2}  {C2}/{C4}/ \\{C2}\\  {C1}┃  {C3}uptime   {C4}{UP:<18} {C1}┃
┃{C2} {C2}/{C4}(   ){C2}\\ {C1}┃  {C3}kernel   {C4}{K:<18} {C1}┃
┃{C2}  {C2}^{C4}`{C4}~{C4}'{C2}^  {C1}┃  {C3}ram      {C4}{RAM:<18} {C1}┃
┗━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛{R}\n",
        C1=c1, C2=c2, C3=c3, C4=c4, R=r,
        H=host_line, OS=os_name, UP=uptime, K=kernel, RAM=ram
    );

    // Выводим всё за один системный вызов
    let _ = std::io::stdout().write_all(output.as_bytes());
}
