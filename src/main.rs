use std::fs;
use std::io::{self, Read};
use std::cmp::max;

fn main() -> io::Result<()> {
    let c1 = "\x1b[38;2;0;135;255m";
    let c2 = "\x1b[38;2;0;175;255m"; 
    let c3 = "\x1b[38;2;0;215;255m"; 
    let c4 = "\x1b[38;2;0;255;255m";
    let r = "\x1b[0m";
    let bold = "\x1b[1m";

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

    let values = [&os_name, &uptime, &kernel, &ram];
    let labels = ["os      ", "uptime  ", "kernel  ", "ram     "];
    let mut max_width = host_line.len();
    for (i, v) in values.iter().enumerate() {
        max_width = max(max_width, labels[i].len() + v.len());
    }
    max_width += 2;
    let top_bar = "━".repeat(max_width + 1);

    println!("{c1}┏━━━━━━━━━┳{top_bar}┓{r}");
    println!("{c1}┃{r}   .~.   {c1}┃{r} {bold}{H:<W$}{r}{c1}┃{r}", H=host_line, W=max_width, c1=c1, r=r, bold=bold);
    println!("{c2}┃{r}   /V\\   {c2}┃{r} os      {OS:<W$}{c2}┃{r}", OS=os_name, W=max_width - 8, c2=c2, r=r);
    println!("{c3}┃{r}  // \\\\  {c3}┃{r} uptime  {UP:<W$}{c3}┃{r}", UP=uptime, W=max_width - 8, c3=c3, r=r);
    println!("{c4}┃{r} /(   )\\ {c4}┃{r} kernel  {K:<W$}{c4}┃{r}", K=kernel, W=max_width - 8, c4=c4, r=r);
    println!("{c4}┃{r}  ^`~'^  {c4}┃{r} ram     {RAM:<W$}{c4}┃{r}", RAM=ram, W=max_width - 8, c4=c4, r=r);
    println!("{c4}┗━━━━━━━━━┻{top_bar}┛{r}", top_bar=top_bar, c4=c4, r=r);

    Ok(())
}
