use std::env;

use jiang_mini_grep::run;
use jiang_mini_grep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap();

    run(config).unwrap()
}
