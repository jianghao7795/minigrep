use std::env;
use std::process;

use jiang_mini_grep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("输入错误: {err}");
        process::exit(1);
    });

    if let Err(e) = jiang_mini_grep::run(config) {
        println!("报错: {e}");
        process::exit(1);
    }

    let v = Some(3);
    if let Some(e) = v {
        println!("i32 is {}", e);
    }
}
