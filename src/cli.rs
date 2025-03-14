use clap::{Command, Arg};

pub struct Args {
    pub usernames: Vec<String>,
    pub token: Option<String>,
    pub save_path: Option<String>,
}

pub fn parse_args() -> Args {
    let matches = Command::new("Github CLI Tool")
        .version("1.0")
        .author("Chad DeGange")
        .about("Fetches top 10 starred repositories from Github users")
        .arg(Arg::new("username").required(true).num_args(1..)) 
        .arg(Arg::new("token").long("token").num_args(1))
        .arg(Arg::new("save").long("save").num_args(1))
        
        .get_matches();

    Args {
      usernames: matches.get_many::<String>("username")
      .unwrap()
      .map(String::as_str)
      .collect(),
  
  token: matches.get_one::<String>("token").map(|s| s.as_str()),
  save_path: matches.get_one::<String>("save").map(|s| s.as_str()),
  
    }
}
