use std::{process::Command, thread::sleep, time};
use argparse::{ArgumentParser, Store};
use shlex::Shlex;
use tokio;

extern crate argparse;

#[tokio::main]
async fn main() {
    let mut url = String::new();
    let mut command = String::new();
    let mut interval_str = "3".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Monitor a URL for page changes");
        ap.refer(&mut url)
            .add_argument("url", Store, "URL of the page to monitor");
        ap.refer(&mut command)
            .add_argument("command", Store, "Command to run when changes are detected");
        ap.refer(&mut interval_str)
            .add_option(&["-i", "--interval"], Store, "Interval (in seconds) between each check");

        ap.parse_args_or_exit();
    }

    // processing arguments
    let interval = interval_str.parse::<u64>().unwrap();
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