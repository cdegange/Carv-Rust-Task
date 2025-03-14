use clap::{Command, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
    Command::new("Github CLI Tool")
        .version("1.0")
        .author("Chad DeGange")
        .about("Fetches top 10 starred repositories from Github users")
        .arg(Arg::new("username").required(true).num_args(1..))
        .arg(Arg::new("token").long("token").num_args(1))
        .arg(
            Arg::new("save")
                .long("save")
                .num_args(1)
                .help("Save output to file in JSON or TOML format"),
        )
        .get_matches()
}
