use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::collections::HashMap;
use std::process;


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: <names file> <hosts file>");
        process::exit(1);
    }

    let filename = &args[1];
    let hosts_file = &args[2];

    let mut out_file = File::create("out.txt").unwrap();
    let mut hosts: HashMap<String, Vec<String>> = HashMap::new();


    let mut contents: HashMap<String, Vec<String>> = HashMap::new();
    let mut results: HashMap<String, Vec<String>> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(entry) = line {
                let split_line:Vec<&str> = entry.split(",").collect();

                match contents.get_mut::<str>(&split_line[1]) {
                    Some(found) => {
                        found.push(split_line[0].to_string());
                    },
                    None => {
                        contents.insert(split_line[1].to_string(), vec![split_line[0].to_string()]);
                    }
                }
            };
        }
    }

    if let Ok(lines) = read_lines(hosts_file) {
        for line in lines {
            if let Ok(entry) = line {
                let split_line:Vec<&str> = entry.split(",").collect();
                match hosts.get_mut::<str>(&split_line[1]) {
                    Some(found) => {
                        found.push(split_line[0].to_string());
                    },
                    None => {
                        hosts.insert(split_line[1].to_string(), vec![split_line[0].to_string()]);
                    }
                };
            };
        }
    }
    
    for (domain, hashes) in contents.iter(){
        for hash in hashes {
            
            match hosts.get_mut::<str>(hash) {
                Some(res) => {
                    match results.get_mut::<str>(&domain) {
                        Some(found) => {
                            found.extend(res.to_vec());
                        },
                        None => {
                            results.insert(domain.to_string(), res.to_vec());
                        }
                    }
                },
                None => {
                    println!("Hash not found!");
                }
            }
        }
    }

    for (key, value) in results.iter(){
        writeln!(out_file, "{}:{}", key, value.join(",")).unwrap();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}