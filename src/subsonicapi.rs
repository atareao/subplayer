use rand::prelude::*;
use md5::compute;
use roxmltree::Document;


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
        let url = format!("{}/rest/ping.view?u={}1&t={:x}&s={}&v={}&c={}",
                          &self.url, &self.username, token, salt, "1.16",
                          "ncli");

        let response = reqwest::get(url)
            .await.unwrap()
            .text()
            .await.unwrap();
        println!("{}", &response);
        let doc = Document::parse(&response).unwrap();
        let root = doc.root().first_child().unwrap().attribute("status").unwrap();
        println!("{}", root);
        "".to_string()
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
