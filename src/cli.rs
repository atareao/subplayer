use clap::{Command, Arg};

pub fn main_cli(name: &str, description: &str, version: &str, authors: &str){
    let matches = Command::new(name)
        .author(authors)
        .version(version)
        .about(description)
        .arg_required_else_help(true)
        .subcommand(Command::new("config")
                    .about("Config")
                    .arg(Arg::new("url")
                         .short('u')
                         .long("url")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("username")
                         .short('n')
                         .long("username")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("password")
                         .short('p')
                         .long("password")
                         .required(true)
                         .takes_value(true)
                         )
            )
        .get_matches();
    if let Some(sub) = matches.subcommand_matches("config"){
        let url = sub.value_of("url").unwrap();
        let username = sub.value_of("username").unwrap();
        let password = sub.value_of("password").unwrap();
        println!("{}, {}, {}", url, username, password);
    }
}
