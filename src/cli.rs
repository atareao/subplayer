use clap::{Command, Arg, AppSettings};

pub fn main_cli(name: &str, description: &str, version: &str, authors: &str){
    let matches = Command::new(name)
        .author(authors)
        .version(version)
        .about(description)
        .get_matches();
}
