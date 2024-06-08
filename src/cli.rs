use clap::{Command, Arg};

pub fn parse_command_line_arguments() -> clap::ArgMatches {
    Command::new("ruskgpt")
        .version(env!("CARGO_PKG_VERSION"))
        .author("255doesnotexist <chenyejin2004@stumail.ysu.edu.cn>")
        .about("Yet another async AskGPT CLI client powered by Rust.")
        .arg(Arg::new("question")
            .help("The question to ask GPT")
            .index(1))
        .arg(Arg::new("config")
            .long("config")
            .short('c')
            .value_parser(clap::value_parser!(String))
            .help("Path to the configuration file"))
        .arg(Arg::new("set")
            .long("set")
            .value_parser(clap::value_parser!(String))
            .help("Set a configuration value in the format key=value"))
        .arg(Arg::new("edit")
            .short('e')
            .long("edit")
            .help("Open the configuration file in the default editor")
            .action(clap::ArgAction::SetTrue)) // 使用 SetTrue action
        .get_matches()
}
