use std::{env, fs::File, io::{BufRead, BufReader}, sync::{Arc, Mutex}};
use sha2::{Sha256, Digest as Sha256Digest};
use sha1::Sha1;
use md5;
use colored::*;
use std::thread;

enum HashType {
    Sha256,
    Sha1,
    Md5,
}

fn parse_hash_type(input: &str) -> Option<HashType> {
    match input.to_lowercase().as_str() {
        "sha256" => Some(HashType::Sha256),
        "sha1" => Some(HashType::Sha1),
        "md5" => Some(HashType::Md5),
        _ => None,
    }
}

fn compute_hash(input: &str, hash_type: &HashType) -> String {
    match hash_type {
        HashType::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        HashType::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(input.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        HashType::Md5 => format!("{:x}", md5::compute(input.as_bytes())),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("{} {}", "Usage:".yellow(), format!("{} <sha256|sha1|md5> <target_hash> [--wordlist filename]", args[0]));
        std::process::exit(1);
    }

    let hash_type = parse_hash_type(&args[1]).expect("Invalid hash type. Use sha256, sha1, or md5.");
    let target_hash = args[2].to_lowercase();

    // Wordlist file support
    let mut wordlist = "rockyou.txt".to_string();
    if args.len() > 4 && args[3] == "--wordlist" {
        wordlist = args[4].clone();
    }

    let file = File::open(&wordlist).expect(&format!("Could not open {}", wordlist));
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let target_hash = Arc::new(target_hash);
    let found = Arc::new(Mutex::new(None));

    // Multithreading
    let num_threads = 4; // Adjust as needed
    let chunk_size = (lines.len() + num_threads - 1) / num_threads;

    let mut handles = vec![];

    for chunk in lines.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let target = Arc::clone(&target_hash);
        let found = Arc::clone(&found);
        let hash_type = hash_type_to_static(&hash_type);

        let handle = thread::spawn(move || {
            for word in chunk {
                let hash = compute_hash(&word, &hash_type);
                if hash == *target {
                    let mut f = found.lock().unwrap();
                    *f = Some(word.clone());
                    return;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    let result = { found.lock().unwrap().clone() };

    if let Some(name) = result {
        println!("{} {}", "Match found!".green().bold(), format!("Name: {}", name).cyan());
    } else {
        println!("{}", "No match found.".red().bold());
    }
    
}

// Clone-friendly hash type
fn hash_type_to_static(hash_type: &HashType) -> HashType {
    match hash_type {
        HashType::Sha256 => HashType::Sha256,
        HashType::Sha1 => HashType::Sha1,
        HashType::Md5 => HashType::Md5,
    }
}
