use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // eprintln!会将标准错误输出到屏幕，标准输出则写入后面跟着的文件
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], minigrep::search_insensitive(query, content))
    }
}
