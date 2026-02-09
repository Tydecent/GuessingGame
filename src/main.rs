use std::env;
use std::io;
use std::io::Write;
use std::process::exit;

use rand::Rng; // 随机数生成器

// 从环境变量获取版本信息（编译时注入）
const BUILD_TIME: &str = env!("BUILD_TIME");
const GIT_HASH: &str = env!("GIT_HASH");

fn main() {
    let args: Vec<String> = env::args().collect(); // 获取所有启动参数
    let mut _debugmode = false; // 调试模式标志位 - 保留备用
    let mut custom_number: (bool, u8) = (false, 0); // 自定义数字标志位
    let mut easter_egg_1 = false; // 彩蛋#1标志位

    // 遍历参数
    let mut i = 1;
    while i < args.len() {
        let now_arg = &args[i];

        match now_arg.as_str() {
            // 调试模式
            "-debug" => _debugmode = true,

            // 彩蛋#1
            "taskarg" => easter_egg_1 = true,

            // 自定义数字
            "-num" => {
                if i + 1 < args.len() {
                    let num = &args[i + 1];

                    if let Ok(n @ 1..=100) = num.parse::<u8>() {
                        let num: u8 = n;

                        custom_number = (true, *&num);

                        // 跳过下一个参数，因为下一个参数是值
                        i += 1;
                    } else {
                        println!("错误：参数-num的无效输入{}", num);
                    }
                } else {
                    println!("错误：参数-num未指定值");
                }
            }

            // 查询版本
            "-version" => {
                println!(
                    "{} {} ({} {})",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                    GIT_HASH,
                    BUILD_TIME
                );
                exit(0);
            }

            other => println!("未知的参数：{}", other),
        }
        i += 1;
    }

    println!("欢迎来到猜数字游戏");

    // 生成随机数
    let secret_number: u8 = if easter_egg_1 {
        1
    } else if custom_number.0 {
        *&custom_number.1
    } else {
        rand::rng().random_range(1..100)
    };

    // 猜错计数
    let mut cycle_count: u32 = 0;

    loop {
        print!("请输入你的猜测：");
        io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新缓冲区（由于print!宏没有换行符，故行缓冲模式不会自动刷新缓冲区）

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("?");

        // 将字符串型转换为整形，解析失败时提示并继续循环
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字！");
                continue;
            }
        };

        // 判断
        if guess > secret_number {
            println!("太大了！\n");
            cycle_count += 1;
        } else if guess < secret_number {
            println!("太小了！\n");
            cycle_count += 1;
        } else if guess == secret_number {
            println!("你猜对了！你之前猜错了{cycle_count}次。");
            break;
        }
    }

    println!("按回车退出...");
    let mut _exit = String::new();
    io::stdin().read_line(&mut _exit).ok();
}
