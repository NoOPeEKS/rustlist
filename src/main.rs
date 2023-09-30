#![allow(unused)]
use clap::Parser;
use reqwest::StatusCode;
use rustlist::get_dict_line;
use colored::Colorize;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::{fs, fmt::format};

#[derive(Parser)]
struct Cli{
    url: String,
    dictionary_path: String,
    num_threads: usize,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    println!("URL: {}\n Dictionary: {}", args.url, args.dictionary_path);

    let directories: String = fs::read_to_string(args.dictionary_path)
                                .expect("Could not open dictionary!");
    
    let directories: Vec<&str> = rustlist::get_dict_line(&directories);

    let pool = ThreadPoolBuilder::new().num_threads(args.num_threads).build().unwrap();

    pool.install(|| {
        directories.par_iter().for_each(|line| {
            let url = format!("{}/{}", args.url, line);
            let response = reqwest::blocking::get(&url).unwrap();
            if response.status() == StatusCode::OK {
                println!("{} ============> /{}", "200 OK".green(), line.yellow());
            }
        });
    });
    
}

