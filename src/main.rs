use std::net::TcpListener;
use std::io::Read;
use std::collections::HashMap;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::str::from_utf8;
use std::process::Command;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        let mut git = Command::new("git");
        let res = git.args(["pull", "--rebase", "origin", "main"]).output();
        println!("{:?}", res);
        // println!("Request: {}",  )// (&buffer[..]));
        // let serialized = String::from_utf8_lossy(&buffer[..]);
        // println!("{:?}", serialized)
        // let des: Hook = serde_json::(&serialized).unwrap();
        // println!("{:?}", des);
        // let mut map = HashMap::new();
        // map.insert("content", "Successfully built and deployed app");
        // let client = reqwest::Client::new();
        // let url = "https://discord.com/api/webhooks/924634783785558028/MOvndSrWsSN0hJZMnYpcTy65KjHAxoBQfoTBQwwxesdb264JkBh48aKhhahAzmFtq1J7";
        // let res = client.post(url).json(&map).send().await;

        // match res.unwrap().status() {
        //     StatusCode::OK => println!("success!"),
        //     StatusCode::PAYLOAD_TOO_LARGE => {
        //         println!("Request payload is too large!");
        //     }
        //     s => println!("Received response status: {:?}", s),
        // };
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Hook {
    url: String
}
