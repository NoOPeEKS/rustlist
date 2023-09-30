#![allow(unused)]
use clap::Parser;
use reqwest::StatusCode;
use rustlist::get_dict_line;
use colored::Colorize;

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
    
    let directories: Vec<&str> = rustlist::get_dict_line(&directories);

    for line in directories {
        let url = format!("{}/{}", args.url, line);
        let response = reqwest::get(url)
            .await
            .unwrap();
        if response.status() == StatusCode::OK{
            println!("{} ============> /{}", "200 OK".green(), line.yellow());
        }

    }
    
}

