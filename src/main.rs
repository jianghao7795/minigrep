use std::env;
use std::process;

// use minigrep::run;
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("输入错误: {}", err);
        process::exit(1); //退出代码终止当前进程
    });

    if let Err(e) = minigrep::run(config) {
        println!("运行出错: {e}");
        process::exit(1); //退出代码终止当前进程
    }
}
