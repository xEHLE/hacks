extern crate clap;
extern crate reqwest;
extern crate tokio;
extern crate native_tls;

use clap::{Arg, App};
use std::string::String;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use futures::{stream, StreamExt};
use hyper::{
    client::HttpConnector,
    Body, Client, Method, Request, StatusCode, Uri,
};
use hyper_tls::{self, HttpsConnector};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let matches = App::new("Httpgrep")
        .version("0.0.1")
        .about("Takes a file with domains, requests each domains with an optional path and then greps the response for a specified value")
        .arg(Arg::with_name("inputFile")
                 .short("f")
                 .long("inputFile")
				 .takes_value(true)
				 .required(true)
                 .help("A list of domains"))
        .arg(Arg::with_name("path")
                 .short("p")
                 .long("path")
                 .takes_value(true)
                 .help("Optional path argument"))
        .arg(Arg::with_name("grepValue")
                 .short("g")
                 .long("grepValue")
				 .takes_value(true)
				 .required(true)
				 .help("Value to match in the response"))
		.arg(Arg::with_name("threads")
                 .short("t")
                 .long("threads")
				 .takes_value(true)
				 .help("Specify thread count (default: 10)"))
		.arg(Arg::with_name("ignorecerts")
                 .short("k")
                 .long("ignorecerts")
				 .help("Ignores certificate validation"))
        .get_matches();

	
	let myfile = matches.value_of("inputFile").unwrap();
	let path= String::from(matches.value_of("path").unwrap_or(""));
	let urls = readlines(myfile, path);

	let ignorecerts = matches.is_present("ignorecerts");

	let threadcount: usize = matches.value_of("threads").unwrap_or("10").parse().unwrap();
    
    let grep = String::from(matches.value_of("grepValue").expect("could not get grepValue"));

	

	let client = reqwest::Client::builder().danger_accept_invalid_certs(ignorecerts).build().expect("error building request client");


	let bodies = stream::iter(urls)
		.map(|url| {
			let client = &client;
			let requrl = reqwest::Url::parse(&url).expect("error parsing url");
			
			async move {
				let res = client.get(requrl).send().await;
				Response { body: res, url:  url }
			}
		})
		.buffer_unordered(threadcount);
	bodies
		.for_each(|b| {
			async {
				match b.body {
					Ok(bod) => {
						let res_body = bod.text().await.unwrap();
						if res_body.contains(&grep) {
							println!("{}", b.url);
						} else {
							return;
						}
					},
					Err(e) => println!("Error: {}", e)
				}
				
			}
		})
		.await;
}

#[derive(Debug)]
struct Response {
	body: std::result::Result<reqwest::Response, reqwest::Error>,
	url: String,
}

fn readlines(filename: impl AsRef<Path>, path: String) -> Vec<String> {
	let file = fs::File::open(filename).expect("error opening file");
	let buf = BufReader::new(file);
	buf.lines()
		.map(|l| {
			let url = l.expect("could not parse line");
			if !url.ends_with("/") {
				format!("{}/{}",url,path)
			} else {

				format!("{}{}",url,path)
			}

		})
		.collect()
}



