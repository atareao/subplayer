use sunk::{Client, Streamable};
use sunk::song::Song;
use dotenv::dotenv;
use std::env;
use libmpv::Mpv;
use md5::compute;
use rand::prelude::*;

fn main() {
    dotenv().ok();
    let url = env::var("NV_URL").expect("URL not configured");
    let user = env::var("NV_USER").expect("USER not configured");
    let password = env::var("NV_PASSWORD").expect("PASSWORD not configured");
    println!("url: {}, user: {}, password: {}", url, user, password);
    let salt = gen_salt(8);
    println!("salt: {}", salt);
    let token = compute(format!("{}{}", password, salt));
    println!("token: {}{} - {:x}", password, salt, token);

    let client = Client::new(&url, &user, &password).unwrap();
    //let mpv = Mpv::new().unwrap();

    let random_songs = Song::random(&client, 20).unwrap();
    for song in random_songs{
        let url = song.stream_url(&client).unwrap();
        println!("URl: {}", url);
    }
    println!("Hello, world!");

}

fn gen_salt(length: u8) -> String{
    let chain = "abcdefghijklmn0pqrstuwxyz";
    let mut result = "".to_string();
    let mut rng = rand::thread_rng();
    for _i in 0..length{
        let selected = rng.gen_range(0..chain.len());
        let slice = &chain[(selected - 1) .. selected];
        result = format!("{}{}", &result, slice);
    }
    result
}
