mod cli;
mod config;
mod subsonicapi;

use config::Config;
use subsonicapi::Client;

#[tokio::main]
async fn main() {
    let mconfig = Config::load_config();
    let url = mconfig.url;
    let user = mconfig.username;
    let password = mconfig.password;
    println!("url: {}, user: {}, password: {}", url, user, password);

    let client = Client::new(&url, &user, &password);
    let result = client.ping().await;
    println!("{:?}", result);
    let result = client.scan_library();
    println!("{:?}", result);
    //let mpv = Mpv::new().unwrap();

    //let asong = Song::get(&client, 1).unwrap();
    //println!("Title: {}", asong.title);

}

