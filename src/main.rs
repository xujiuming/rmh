use std::ffi::OsString;

use clap::{arg, Command};

fn cli() -> Command {
    Command::new("rmh")
        .about("ming")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("add")
                .about("Clones repos")
                .arg(arg!(<HOST> "host name"))
                .arg_required_else_help(true),
        )

        .subcommand(
            Command::new("list")
                .about("pushes things")
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("del")
                .about("adds things")
                .arg_required_else_help(true),
        )
}


fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", _sub_matches)) => {
            println!("Adding");
        }
        Some(("list", _sub_matches)) => {
            println!("list");
        }
        Some(("del", _sub_matches)) => {
            println!("del");
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {ext:?} with {args:?}");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }

    // Continued program logic goes here...
}
