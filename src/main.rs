use std::{process::Command, thread::sleep, time};
use clap::{command, Parser};
use shlex::Shlex;
use tokio;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    url: String,

    command: String,

    #[arg(short, long, default_value_t = 3)]
    interval: u64,
}


#[tokio::main]
async fn main() {

    let args = Args::parse();
    let url = args.url;
    let command = args.command;
    let interval = args.interval;

    // processing arguments
    let stripped_url = strip_protocol(url.clone());
    // split the command into base command & args
    let mut parts = Shlex::new(command.as_str());
    
    let program = parts.next().unwrap();
    let args: Vec<String> = parts.collect();

    let mut old_page = send_request(url.clone()).await.unwrap();
    println!("Sent initial request to {}", stripped_url);
    
    let mut page;
    loop {
        sleep(time::Duration::from_secs(interval));
        
        page = send_request(url.clone()).await.unwrap();
        if !page.eq(&old_page) {
            println!("Page change detected on {}!", stripped_url);
            Command::new(program.clone())
                .args(&args)
                .output()
                .expect(format!("Failed to execute: {}", command).as_str());

            old_page = page;
        }
    }
}

async fn send_request(url: String) -> reqwest::Result<String> {
    reqwest::get(url)
    .await?
    .text()
    .await
}

fn strip_protocol(url: String) -> String {
    if let Some(stripped) = url.strip_prefix("https://") {
        return stripped.to_string();
    } else if let Some(stripped) = url.strip_prefix("http://") {
        return stripped.to_string();
    } else {
        return url;
    }
}