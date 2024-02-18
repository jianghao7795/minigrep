use std::env;

use jiang_mini_grep::run;
use jiang_mini_grep::Config;

fn main() {
    let args = env::args(); // Args 实现了 Iterator
    let config = Config::build(args).unwrap();

    run(config).unwrap()
    // unwrap_or_else 错误处理
    // let config = Config::build(env::args()).unwrap_or_else(|err| {
    //     println!("输入错误: {err}");
    //     process::exit(1); //退出代码终止当前进程
    // });

    // let config = Config::build(env::args()).unwrap_or(|e| {
    //     println!("{e}");
    //     process::exit(1);
    // });

    // Err(str) 错误信息
    // if let Err(e) = jiang_mini_grep::run(config) {
    //     println!("运行出错: {e}");
    //     process::exit(1); //退出代码终止当前进程
    // }
}
