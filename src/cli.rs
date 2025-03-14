use clap::{App, Arg};

pub struct Args {
    pub usernames: Vec<String>,
    pub token: Option<String>,
    pub save_path: Option<String>,
}

pub fn parse_args() -> Args {
    let matches = App::new("Github CLI Tool")
        .version("1.0")
        .author("Chad DeGange")
        .about("Fetches top 10 starred repositories from Github users")
        .arg(Arg::with_name("username").required(true).multiple(true))
        .arg(Arg::with_name("token").long("token").takes_value(true))
        .arg(Arg::with_name("save").long("save").takes_value(true))
        .get_matches();

    Args {
        usernames: matches.values_of("username").unwrap().map(String::from).collect(),
        token: matches.value_of("token").map(String::from),
        save_path: matches.value_of("save").map(String::from),
    }
}
