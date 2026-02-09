use std::process::Command;

fn main() {
    // 获取当前时间作为构建时间
    let build_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // 获取Git提交哈希
    match Command::new("git").args(&["rev-parse", "HEAD"]).output() {
        Ok(output) if output.status.success() => {
            let hash = String::from_utf8_lossy(&output.stdout);
            // 取前9位
            let short_hash = &hash[..9.min(hash.len())].trim();
            println!("cargo:rustc-env=GIT_HASH={}", short_hash);
        }
        _ => {
            // 如果不是Git仓库或Git不可用
            println!("cargo:rustc-env=GIT_HASH=unknown");
        }
    }
}
