#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli{
    url: String,
    dictionary_path: String,
}
use std::{fs, fmt::format};

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    println!("URL: {}\n Dictionary: {}", args.url, args.dictionary_path);

    let directories: String = fs::read_to_string(args.dictionary_path)
                                .expect("Could not open dictionary!");
    
    let directories: Vec<&str> = get_dict_line(&directories);

    for line in directories {
        let url = format!("{}/{}", args.url, line);
        let response = reqwest::get(url)
            .await
            .unwrap()
            .text()
            .await;
        println!("{:?}", response);
    }
    
}

fn get_dict_line<'a>(contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
            results.push(line);
    }

    return results;
}