#![allow(unused)]

extern crate prettylog;

use std::net::TcpListener;
use std::io::Read;
use std::process::Command;
use structopt::StructOpt;
use prettylog::*;
use std::io::{self, Write};
use std::str;
use std::collections::HashMap;

const PORT: &str = "7878";

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let addr = format!("{}:{}", &args.bind, PORT);
    let listener = TcpListener::bind(addr).unwrap();
    let msg = format!("Server listening at: {}:{}", &args.bind, PORT);
    info(&msg);
    let mut yarn = Command::new("yarn");
    info(format!("Project {} started", &args.path).as_str());
    
    for stream in listener.incoming() {
        let kill = Command::new("npx").current_dir(&args.path).args(["kill-port", "3000"]).output().expect("Error");
        info("Exit app");
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let post = b"POST / HTTP/1.1\r\n";
        if (buffer.starts_with(post)) {

            let git = Command::new("git")
                .current_dir(&args.path)
                .args(["pull", "--rebase", "origin", "main"])
                .output()
                .expect("failed to execute process");
            let error_m = format!("\x1B[31m{}\x1B[0m", String::from_utf8_lossy(&git.stderr));
            let info_m = format!("\x1B[32m{}\x1B[0m", String::from_utf8_lossy(&git.stdout));
            
            io::stdout().write_all(info_m.as_bytes()).unwrap();
            io::stderr().write_all(error_m.as_bytes()).unwrap();

            info("Building app");
            let build = Command::new("yarn").current_dir(&args.path).arg("build").output().expect("Err!"); //.output().expect("Error!");
            io::stdout().write_all(&build.stdout).unwrap();
            io::stdout().write_all(&build.stderr).unwrap();
            info("Build Finished");
            yarn.current_dir(&args.path).args(["start-bg"]).spawn().expect("Error running app");
        

            let mut map = HashMap::new();
            let name = "QuickRN";
            let msg = format!("Successfully built and deployed {}", name);
            map.insert("content", msg);
            let client = reqwest::Client::new();
            let res = client.post(&args.webhook).json(&map).send().await;
        }
    }
}

#[derive(StructOpt)]
#[structopt(name = "proxim")]
struct Cli {
    // ip address you wish to bind to
    #[structopt(short = "b", long = "bind", default_value = "127.0.0.1")]
    bind: String,
    // Path to Next.js project where you the package.js lives
    #[structopt(short = "p", long = "path")]
    path: String,
    // Discord webook URL
    #[structopt(short = "w", long = "webhook")]
    webhook: String
}
