use std::thread;
use rustc_serialize::json;
use url::Url;
use tungstenite::{connect, Message};
use std::time::SystemTime;
use tokio::time::*;
use colored::Colorize;
use serde::Serialize;
use serde_derive::Deserialize;
use chrono::{Local, Timelike};
use rand::Rng;
use reqwest::Client;
use serde_json;
use reqwest::header;
use spinner::SpinnerBuilder;

#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    captchaToken: String,
    potId: u32,
    iloveu: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub users: Vec<User>,
    #[serde(rename = "discord_webhook")]
    pub discord_webhook: String,
    #[serde(rename = "2captcha")]
    pub n2captcha: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub authorization: String,
    #[serde(rename = "proxy_http")]
    pub proxy_http: String,
}


#[tokio::main]
async fn main(){
    let debug = true;
    let sp = SpinnerBuilder::new("Long Running operation, please wait...".into()).start();
    if !debug {
        sp.message("[2.0.5] Long Running operation, please wait...".into());
    }
    let mut bal = 0;
    let mut potid= 0;

    'outer: loop {
        if debug {
            println!("[{}] Entering debug mode", format!("WARNING").yellow().bold());

        }
        let (mut socket, _) = connect(
            Url::parse("wss://rblxwild.com/socket.io/?EIO=4&transport=websocket").unwrap()
        ).expect("Can't connect");
        let file = "./config/config.json";
        let config: Config = serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
        // get length of Users

        let value = &("42[\"authentication\",{\"authToken\":\"".to_owned()+ &*config.users[0].authorization + &*"\",\"clientTime\":1651530049953}]".to_owned());
        let auths = vec!["40", "42[\"chat:subscribe\",{\"channel\":\"EN\"}]", "42[\"cases:subscribe\"]", value];
        for auth in auths {
            socket.write_message(Message::Text(auth.to_string())).unwrap();
            let msg = socket.read_message().unwrap();
            let str_msg = msg.to_string();
            if str_msg.contains("authentication") {
                let balance_split = str_msg.split("\"id\":");
                for i in balance_split {
                    if i.contains("prize") {
                        potid = i.split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                    }

                }
                let balance_split = str_msg.split("\"balance\":");
                for i in balance_split {
                    if i.contains("role") {
                        bal = i.split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                    }
                }
            }
            if debug {println!("{}", str_msg);}
        }

        loop {
            // check or
            if check_if_its_7am()   {
                let string_json = "{\"content\":\"@everyone Daily recap ||https://rblxwild.com/?modal=trading-cashier&type=WITHDRAW||\",\"embeds\":[{\"title\":\"Todays Profits\",\"color\":5814783,\"fields\":[{\"name\":\"Account Balance\",\"value\":\"💸 ".to_owned()+ &*bal.to_string() + &*"\",\"inline\":true},{\"name\":\"USD Balance\",\"value\":\"💰 $".to_owned() +&*((bal/2)/100).to_string()+"\",\"inline\":true}],\"thumbnail\":{\"url\":\"https://i.imgur.com/BeGs0RY.png\"}}],\"avatar_url\": \"https://discohook.org/static/discord-avatar.png\",\"attachments\":[]}";
                let client = Client::new();
                client.post(&config.discord_webhook)
                    .header("Content-Type", "application/json")
                    .body(string_json)
                    .send()
                    .await.unwrap();

                for user in config.users.iter() {
                    let mut headers = header::HeaderMap::new();
                    headers.insert("Content-Type", "application/json".parse().unwrap());
                    headers.insert("Pragma", "no-cache".parse().unwrap());
                    headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
                    headers.insert("Authorization", user.authorization.parse().unwrap());
                    headers.insert("Accept-Language", "en-US,en;q=0.9".parse().unwrap());
                    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
                    headers.insert("Cache-Control", "no-cache".parse().unwrap());
                    headers.insert("Host", "rblxwild.com".parse().unwrap());
                    headers.insert("Origin", "https://rblxwild.com".parse().unwrap());
                    headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.2 Safari/605.1.15".parse().unwrap());
                    headers.insert("Content-Length", "64".parse().unwrap());
                    headers.insert("Connection", "keep-alive".parse().unwrap());
                    headers.insert(header::COOKIE, "__mmapiwsid=e6754f24-f581-4b02-9295-dbf0f13ad717:a6586f216a80022e6cf74ada57130f99744db842; session=s%3A7M5xJJq41sM04a6HsrpKlAaCGYxOhNdF.zCzf%2FmqCvofCLX4YBmC7a5H25%2Fs8vPbpxn6B%2BahoXEM; _gcl_au=1.1.583011129.1654289048".parse().unwrap());

                   Client::new()
                        .post("https://rblxwild.com/api/trading/robux/request-exchange")
                        .headers(headers)
                        .body("{\"type\":\"DEPOSIT\",\"amount\":100,\"instant\":false,\"dummyAssetId\":0}")
                        .send().await;

                    let mut headers = header::HeaderMap::new();
                    headers.insert("Content-Type", "application/json".parse().unwrap());
                    headers.insert("Authorization", user.authorization.parse().unwrap());
                    let client = Client::new();
                    let response = client.post("https://rblxwild.com/api/trading/robux/request-exchange")
                        .headers(headers)
                        .body("{\"type\":\"WITHDRAW\",\"amount\":".to_owned() + &*bal.to_string() + ",\"instant\":false,\"dummyAssetId\":0}")
                        .send().await;
                    if response.is_ok() {
                        sp.update("Withdrawal Successful 📩".parse().unwrap());
                        continue 'outer;
                    } else {
                        sp.update("Withdrawal Failed ❎ ".parse().unwrap());
                        continue 'outer;
                    }
                }
                sleep(Duration::from_secs(60)).await;

            }

            let msg = socket.read_message();
            // check if error happened
            if let Err(e) = msg {
                if debug {
                    println!("{:?}", e)
                }
                continue 'outer;
            }
            if msg.as_ref().unwrap().is_close() {
                continue 'outer;
            }
            let msg = msg.unwrap();
            if debug {
                println!("{:?}", msg);
            }
            if msg.to_string() == "2" {
                socket.write_message(Message::Text("3".to_string())).unwrap();
                if debug {
                    println!("Sent 3");
                }
            }
            if msg.to_string().contains("updatePotVariables") {
                let new_prize = msg.to_string();

                if !debug {

                    sp.update("Pool Prize 💸 ".to_owned() + &new_prize.split("newPrize\":").collect::<Vec<&str>>()[1].to_string().split(",\"new").collect::<Vec<&str>>()[0] + &*" | Balance 💁 ".to_owned() + &*bal.to_string());
                }
                let rand_num = rand::thread_rng().gen_range(0..100);
                
                if rand_num <2  {
                    socket.write_message(Message::Text("42[\"crash:bet\",{\"betAmount\":5,\"autoCashout\":1.01}]".to_string())).unwrap();

                }
            }
            // 42["user:updateBalance",{"value":1417,"time":1654808912089}]
            if msg.to_string().contains("user:updateBalance") {
                let bal_info = msg.to_string().split("\"value\":").collect::<Vec<&str>>()[1].to_string();
                let bal_info2 = bal_info.split(",\"time").collect::<Vec<&str>>()[0];
                bal = bal_info2.parse::<i32>().unwrap();
            }

            if msg.to_string().contains("ENDING") && msg.to_string().contains("newState") {
                for user in config.users.iter() {
                    let user = user.clone();
                    if debug {
                        println!("{:?} joining", user);
                    }
                    tokio::spawn(join(potid, user.authorization,config.clone().n2captcha,debug));
                }
                if debug {
                    println!("Now that we finished joining, we will add one digit to pool id. Current pot id {} New pot id {}", potid, potid+1)
                }
                potid += 1;
                if debug{
                    println!("New pot id {}", potid)
                }
            }
        }
    }
}
fn check_if_its_7am() -> bool {
    let now = Local::now();
    if now.hour() == 21 && now.minute() == 0 {
        return true;
    }
    return false;
}


async fn join(potid: i32, authorization: String, captcha_token: String, debug: bool)  {
    let client = Client::new();
    let result = client.post("https://2captcha.com/in.php?key=".to_owned()+ &*captcha_token +"&method=hcaptcha&sitekey=30a8dcf5-481e-40d1-88de-51ad22aa8e97&pageurl=https://2captcha.com/demo/hcaptcha")
        .send()
        .await.unwrap();
    let id: String = result.text().await.unwrap().split("|").collect();
    let sw = SystemTime::now();
    loop {
        let url = format!("https://2captcha.com/res.php?key={}&action=get&id={}",captcha_token.clone(), id[2..id.len()].to_string());
        let result = client.get(&url)
            .send()
            .await.unwrap();
        let result: String = result.text().await.unwrap().split("|").collect();
        if debug {
            println!("{}", result);
        }
        if result.contains("OK") {
            let client = Client::new();
            let object = TestStruct {
                captchaToken: result[2..result.len()].to_string(),
                potId: potid as u32,
                iloveu: true,
            };
            let encoded = json::encode(&object).unwrap();
            if debug {
                println!("{}", encoded);
            }
            let res = client.post("https://rblxwild.com/api/events/rain/join")
                .header("Content-Type", "application/json")
                .header("authorization", authorization)
                .body(encoded)
                .send()
                .await.unwrap();

            if debug {
                println!("{:?}", res);
            }
            return;

        }
        if SystemTime::now().duration_since(sw).unwrap().as_secs() > 120 {
            return;
        }
        thread::sleep(Duration::from_secs(1));
    }
}