extern crate prettylog;

use std::net::TcpListener;
use std::io::Read;
use serde::{Serialize, Deserialize};
// use std::str::from_utf8;
use std::process::Command;
use structopt::StructOpt;
use prettylog::*;

const PORT: &str = "7878";

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let addr = format!("{}:{}", &args.bind, PORT);
    let listener = TcpListener::bind(addr).unwrap();
    let msg = format!("Server listening at: {}:{}", &args.bind, PORT);
    info(&msg);

    // for stream in listener.incoming() {
    //     let mut stream = stream.unwrap();
    //     let mut buffer = [0; 1024];

    //     stream.read(&mut buffer).unwrap();
    //     let git = Command::new("git").args(["pull", "--rebase", "origin", "main"]).output();
    //     println!("{:?}", String::from_utf8(git.unwrap().stderr));
    //     let build = Command::new("yarn").args(["build"]).output();
    //     println!("{:?}", String::from_utf8(build.unwrap().stderr));

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
    // }
}

#[derive(StructOpt)]
#[structopt(name = "proximo")]
struct Cli {
    #[structopt(short = "b", long = "bind", default_value = "127.0.0.1")]
    bind: String
}
