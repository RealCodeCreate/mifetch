use std::fs;
use std::io::{self, Read};
use std::cmp::max;

fn main() -> io::Result<()> {
    // Цвета ANSI (Яркий голубой/циан)
    let c = "\x1b[1;36m"; // Bold Cyan (Яркий голубой)
    let r = "\x1b[0m";    // Reset
    let white = "\x1b[1;37m"; // Bold White для акцентов

    // 1. Собираем данные
    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let mut host_buf = [0u8; 64];
    let host_name = if let Ok(mut f) = fs::File::open("/proc/sys/kernel/hostname") {
        let n = f.read(&mut host_buf).unwrap_or(0);
        String::from_utf8_lossy(&host_buf[..n]).trim().to_string()
    } else {
        "unknown".to_string()
    };
    let host_line = format!("{}@{}", user, host_name);

    let mut os_name = "Linux".to_string();
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                os_name = line.replace("PRETTY_NAME=", "").replace('"', "");
                break;
            }
        }
    }

    let uptime_str = fs::read_to_string("/proc/uptime").unwrap_or_default();
    let uptime_sec = uptime_str.split_whitespace().next().unwrap_or("0").parse::<f32>().unwrap_or(0.0) as u64;
    let uptime = format!("{}h {}m", uptime_sec / 3600, (uptime_sec % 3600) / 60);

    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_default().trim().to_string();

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

    // 2. Динамическая ширина
    let labels = ["os      ", "uptime  ", "kernel  ", "ram     "];
    let values = [&os_name, &uptime, &kernel, &ram];
    let mut max_width = host_line.len();
    for (i, v) in values.iter().enumerate() {
        max_width = max(max_width, labels[i].len() + v.len());
    }
    max_width += 2; 
    let top = "━".repeat(max_width + 1);

    // 3. Вывод (Яркая голубая рамка и текст)
    println!("{c}┏━━━━━━━━━┳{top}┓{r}");
    println!("{c}┃{r} {c}  .~.   {r} {c}┃{r} {white}{H:<W$}{r}{c}┃{r}", c=c, r=r, white=white, H=host_line, W=max_width);
    println!("{c}┃{r} {c}  /V\\   {r} {c}┃{r} {c}os      {r}{OS:<W$}{c}┃{r}", c=c, r=r, OS=os_name, W=max_width - 8);
    println!("{c}┃{r} {c} // \\\\  {r} {c}┃{r} {c}uptime  {r}{UP:<W$}{c}┃{r}", c=c, r=r, UP=uptime, W=max_width - 8);
    println!("{c}┃{r} {c}/(   )\\ {r} {c}┃{r} {c}kernel  {r}{K:<W$}{c}┃{r}", c=c, r=r, K=kernel, W=max_width - 8);
    println!("{c}┃{r} {c} ^`~'^  {r} {c}┃{r} {c}ram     {r}{RAM:<W$}{c}┃{r}", c=c, r=r, RAM=ram, W=max_width - 8);
    println!("{c}┗━━━━━━━━━┻{top}┛{r}", c=c, top=top, r=r);

    Ok(())
}
