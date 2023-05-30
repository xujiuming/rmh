use clap::Parser;
use human_panic::setup_panic;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    //安装 human-panic
    setup_panic!();

    let args = Cli::parse();
    let read_file_result = std::fs::read_to_string(&args.path);
    let content = match read_file_result {
        Ok(content) => { content }
        Err(err) => {
            panic!("not found file! {}", err);
        }
    };
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}