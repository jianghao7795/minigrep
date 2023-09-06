// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     // dbg!(args);

//     let (query, file_path) = parse_config(&args);

//     println!("Searching for {}", query);
//     println!("In file {}", file_path);

//     let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
//     println!("With text:\n{contents}");
// }

// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];

//     (query, file_path)
// }

// use std::env;
// use std::fs;
// use std::io::Error;
// use std::process;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     // 对 build 返回的 `Result` 进行处理
//     let config = Config::build(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.file_path);
//     if let Err(e) = run(config) {
//         println!("Application error: {e}");
//         process::exit(1);
//     }
//     let contents =
//         fs::read_to_string(config.file_path).expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
// }

// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     println!("With text:\n{contents}");

//     Ok(())
// }

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}