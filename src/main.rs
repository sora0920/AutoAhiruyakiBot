extern crate reqwest;
extern crate json_flex;
use std::error::Error;
use std::fs;
use std::env;
use std::io::{BufReader, Read};
use reqwest::header::{Authorization, Bearer};
use std::collections::HashMap;
use reqwest::StatusCode;

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();
    let mut json: String = String::new();
    
    for path in paths {
        read_file(&path, &mut json);
        don_post(&json);
        json = "".to_string();
    }
} 

fn read_file(path: &str, mut json: &mut String){
    let mut f = BufReader::new(fs::File::open(path).unwrap());

    match f.read_to_string(&mut json){
        Err(why) => panic!("couldn't read {}: {}", path,
                                                   Error::description(&why)),
        Ok(_) => println!("FileReadOK!")
    };
}

fn don_post(json: &str) {
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

    let resp = client.post(&host)
        .json(&map)
        .header(Authorization(Bearer { token: token }))
        .send()
        .unwrap();

    match resp.status() {
        StatusCode::Ok => println!("{} is done", parse_json["name"].into_string().unwrap().to_string()),
        StatusCode::PayloadTooLarge => {
            println!("{}: Request payload is too large!", parse_json["name"].into_string().unwrap().to_string());
        }
        s => println!("{}: Received response status: {:?}", parse_json["name"].into_string().unwrap().to_string(), s),
    };
}
