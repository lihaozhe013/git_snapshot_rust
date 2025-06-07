use std::env;
use std::fs::{self, File};
use std::io::{self, Write, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(windows)]
const CLEAR_CMD: &str = "cls";
#[cfg(not(windows))]
const CLEAR_CMD: &str = "clear";

// ANSI color codes
const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_BLUE: &str = "\x1b[34m";
const COLOR_MAGENTA: &str = "\x1b[35m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_WHITE: &str = "\x1b[37m";
const COLOR_BOLD: &str = "\x1b[1m";
const COLOR_DIM: &str = "\x1b[2m";
const COLOR_RESET: &str = "\x1b[0m";

#[cfg(windows)]
const USER_ENV: &str = "USERNAME";
#[cfg(not(windows))]
const USER_ENV: &str = "USER";

// 打印带边框的内容
fn print_box(title: &str, content: &str, color: &str) {
    let width = 60;
    let sep = "-".repeat(width - 4);
    println!("\n{}", color);
    println!("+-{}-+", &sep);
    println!("| {:<width$} |", title, width = width - 4);
    println!("+-{}-+", &sep);
    println!("| {:<width$} |", content, width = width - 4);
    println!("+-{}-+\n{}", &sep, COLOR_RESET);
}

// 显示加载动画
fn show_loading(message: &str) {
    let frames = ["-", "\\", "|", "/"];
    for i in 0..8 {
        print!(
            "{}{} {}{}\r",
            COLOR_CYAN,
            frames[i % 4],
            message,
            COLOR_RESET
        );
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100));
    }
    println!();
}

// 打印成功消息
fn print_success(message: &str) {
    println!(
        "\n{}{}[+]{} {}",
        COLOR_GREEN, COLOR_BOLD, COLOR_RESET, message
    );
}

// 打印错误消息
fn print_error(message: &str) {
    println!(
        "\n{}{}[-]{} {}",
        COLOR_RED, COLOR_BOLD, COLOR_RESET, message
    );
}

// 打印菜单标题
fn print_menu(title: &str) {
    // 清屏
    let _ = Command::new(CLEAR_CMD).status();
    println!("\n{}{}[*] Git Snapshot {}",
        COLOR_CYAN, COLOR_BOLD, COLOR_RESET
    );
    println!("{}{}{}\n", COLOR_DIM, title, COLOR_RESET);
}

// 检查 git 是否安装
fn check_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

// 按任意键退出（简单实现，用回车）
fn wait_exit() {
    println!("\n{}> Press Enter to exit...{}", COLOR_DIM, COLOR_RESET);
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

// 获取用户名
fn get_username() -> String {
    env::var(USER_ENV).unwrap_or_else(|_| "unknown".to_string())
}

// 获取格式化时间戳
fn get_timestamp() -> String {
    let now = std::time::SystemTime::now();
    let datetime = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let secs = datetime.as_secs();
    
    // Convert to local time components
    let mut secs_remaining = secs;
    let year = 1970 + (secs_remaining / 31536000);
    secs_remaining %= 31536000;
    let month = 1 + (secs_remaining / 2592000);
    secs_remaining %= 2592000;
    let day = 1 + (secs_remaining / 86400); 
    secs_remaining %= 86400;
    let hour = secs_remaining / 3600;
    secs_remaining %= 3600;
    let min = secs_remaining / 60;
    let sec = secs_remaining % 60;

    format!("{:04}-{:02}-{:02}_{:02}-{:02}-{:02}", 
        year, month, day, hour, min, sec)
}

// 初始化仓库
fn init_repo() {
    print_menu("Repository Initialization");
    show_loading("Initializing Git repository");

    let _ = Command::new("git").arg("init").status();

    // 写 .gitignore
    let _ = fs::write(".gitignore", "log/\nrun.command\ngit_snapshot\n.DS_Store\nnul\n");

    // 创建 log 目录
    #[cfg(windows)]
    let _ = Command::new("cmd").args(&["/C", "mkdir log"]).status();
    #[cfg(not(windows))]
    let _ = Command::new("mkdir").arg("-p").arg("log").status();

    // 空提交
    show_loading("Creating initial commit");
    let _ = Command::new("git").args(&["add", "."]).status();
    let _ = Command::new("git")
        .args(&["commit", "--allow-empty", "-m", "Initial commit"])
        .status();

    print_success("Repository initialization completed");
}

// 执行提交
fn commit_changes() {
    let user = get_username();
    let timebuf = get_timestamp();

    print_menu("Git Snapshot");
    show_loading("Checking for changes");

    let _ = Command::new("git").args(&["add", "."]).status();

    // 检查有没有变更
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
        .expect("failed to run git status");
    if output.stdout.is_empty() {
        print_box("Status", "No changes detected in the repository", COLOR_YELLOW);
        wait_exit();
        return;
    }

    // 提交
    show_loading("Committing changes");
    let msg = format!("{} {}", user, timebuf);
    let _ = Command::new("git").args(&["commit", "-m", &msg]).status();

    // 记录 log
    let logfile = format!("log/{}.log", timebuf);
    let mut file = File::create(&logfile).unwrap();
    writeln!(file, "Commit by {} at {}", user, timebuf).unwrap();

    let status_msg = format!("Changes committed successfully by {}", user);
    print_box("Commit Status", &status_msg, COLOR_GREEN);
    wait_exit();
}

// 检查是否有 .git 目录
fn has_git_repo() -> bool {
    fs::metadata(".git").is_ok()
}

fn main() {
    print_menu("System Check");

    if !check_git_installed() {
        print_error("Git is not installed on your system");
        print_box(
            "Required Action",
            "Please install Git before using this tool",
            COLOR_RED,
        );
        wait_exit();
        return;
    }

    if !has_git_repo() {
        print_box(
            "Repository Check",
            "Current folder is not a Git repository",
            COLOR_YELLOW,
        );
        print!(
            "\n{}>{} Initialize new repository? [y/N]: ",
            COLOR_CYAN, COLOR_RESET
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase().starts_with('y') {
            init_repo();
        } else {
            print_error("Repository initialization cancelled");
            wait_exit();
            return;
        }
    }

    commit_changes();
}
