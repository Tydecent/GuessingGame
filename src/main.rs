use std::io;
use std::io::Write;

use rand::Rng; // 随机数生成器

fn main() {
    println!("欢迎来到猜数字游戏");

    // 生成随机数
    let secret_number: u8 = rand::rng().random_range(1..101);
    
    // 猜错计数
    let mut cycle_count: u32 = 0;

    loop {
        if cycle_count > 0 {

            println!("你猜错了{cycle_count}次。");

        }

        print!("请输入你的猜测：");
        io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新缓冲区（由于print!宏没有换行符，故行缓冲模式不会自动刷新缓冲区）

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("?");

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
        } else {
            println!("如果你成功看到这行文字，游戏开发者向你致敬。");
        }
    }

    println!("按回车退出...");
    let mut _exit = String::new();
    io::stdin().read_line(&mut _exit).ok();
    
}
