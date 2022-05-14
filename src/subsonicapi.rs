use rand::prelude::*;
use md5::compute;
use serde_json::{Value, Map, json};


#[derive(Debug)]
pub struct Client{
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Client{
    pub fn new(url: &str, username: &str, password: &str) -> Self{
        Client{
            url: url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub async fn ping(&self) -> String{
        let salt = Self::gen_salt(8);
        let token = compute(format!("{}{}", &self.password, salt));
        let url = format!("{}/rest/ping.view?u={}&t={:x}&s={}&v={}&c={}&f={}",
                          &self.url, &self.username, token, salt, "1.16",
                          "ncli", "json");

        let response = reqwest::get(url)
            .await.unwrap()
            .text()
            .await.unwrap();
        let parsed: Value = serde_json::from_str(&response).unwrap();
        let data: Map<String, Value> = parsed.as_object().unwrap().clone();
        println!("{:?}", &data);
        //{"subsonic-response": Object({"error": Object({"code": Number(40), "message": String("Wrong username or password")}), "serverVersion": String("0.47.5 (86fe1e3b)"), "status": String("failed"), "type": String("navidrome"), "version": String("1.16.1")})}
        //{"subsonic-response": Object({"serverVersion": String("0.47.5 (86fe1e3b)"), "status": String("ok"), "type": String("navidrome"), "version": String("1.16.1")})}
        if let Some(response) = data.get("subsonic-response"){
            if let Some(status) = response.get("status"){
                if status.as_str().unwrap() == "ok"{
                    return "pong".to_string();
                }
            }
        }
        "ko".to_string()
    }

    pub fn scan_library(&self){

    }

    fn gen_salt(length: u8) -> String{
        let chain = "abcdefghijklmn0pqrstuwxyz";
        let mut result = "".to_string();
        let mut rng = rand::thread_rng();
        for _i in 0..length{
            let selected = rng.gen_range(0..chain.len());
            let slice = &chain[selected..selected + 1];
            result = format!("{}{}", &result, slice);
        }
        result
    }
}
