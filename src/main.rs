extern crate reqwest;
extern crate json_flex;
use std::error::Error;
use std::fs;
use std::env;
use std::io::{BufReader, Read};
use reqwest::header::{Authorization, Bearer};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1]; // 設定ファイルのPathを入れる(host, token, status, [cw], [vis])
    let mut f = BufReader::new(fs::File::open(path).unwrap());
    let mut json = String::new();

    match f.read_to_string(&mut json){
        Err(why) => panic!("couldn't read {}: {}", path,
                                                   Error::description(&why)),
        Ok(_) => println!("FileReadOK!")
    };

    let parse_json = json_flex::decode(json.to_owned());

    let status: String = parse_json["status"].into_string().unwrap().to_string();
    let cw: String = parse_json["spoiler_text"].into_string().unwrap().to_string();
    let vis: String = parse_json["visibility"].into_string().unwrap().to_string();
    let mut host: String = parse_json["host"].into_string().unwrap().to_string();
    let token: String = parse_json["token"].into_string().unwrap().to_string();

    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("status", status);
    map.insert("spoiler_text", cw);
    map.insert("visibility", vis);

    host = format!("https://{}/api/v1/statuses", &host).to_string();

    let _res = client.post(&host)
        .json(&map)
        .header(Authorization(Bearer { token: token }))
        .send();
    println!("{} is done", parse_json["name"].into_string().unwrap().to_string());
} 

