﻿# HashCracker
Just a fun project i did to practice...might be terrible code :D 

HashCracker is a simple, fast, and efficient tool for cracking hash values using different hashing algorithms: SHA-256, SHA-1, and MD5. The tool compares each word in a wordlist to a target hash, attempting to find the original string. It uses multithreading for increased performance.

## Features

- **Supports multiple hash types**: SHA-256, SHA-1, and MD5.
- **Multithreaded**: Cracks hashes faster by dividing the workload across multiple threads.
- **Custom wordlist support**: Allows the user to specify a custom wordlist file.
- **Colorized output**: Results are displayed with color formatting (success is green, failure is red).

## Installation

To use this project, you need to have **Rust** installed on your machine. If you haven't installed Rust, you can follow the instructions on the official website: [Install Rust](https://www.rust-lang.org/learn/get-started).

Once you have Rust installed, clone the repository or copy the code into your project folder.

```bash
git clone <repository-url>
cd <project-folder>
```

Then, you can build and run the program.

## Usage

```bash
cargo run <hash_type> <target_hash> [--wordlist <filename>]
```

### Parameters

- `<hash_type>`: The type of hash to crack. This can be one of the following:
  - `sha256` – For SHA-256 hashes.
  - `sha1` – For SHA-1 hashes.
  - `md5` – For MD5 hashes.
  
- `<target_hash>`: The hash you want to crack (e.g., the hash of a password).

- `--wordlist <filename>` (optional): The path to a custom wordlist file. By default, it uses "rockyou.txt". (u need to download it first and put it into the root-folder since its too big: https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt) 

### Example

```bash
cargo run sha256 5e884898da28047151d0e56f8dc6292773603d0d3f4f2d9b1b1c2c44c55282c3 --wordlist custom_wordlist.txt
```

This command will attempt to find the original string for the given SHA-256 hash `5e884898da28047151d0e56f8dc6292773603d0d3f4f2d9b1b1c2c44c55282c3` using the wordlist `custom_wordlist.txt`.

## Example Output

```
Match found! Name: password123
```

If no match is found, the output will look like:

```
No match found.
```

## Dependencies

This project relies on the following dependencies:

- `sha2`: A library for computing SHA-256 hashes.
- `sha1`: A library for computing SHA-1 hashes.
- `md5`: A library for computing MD5 hashes.
- `colored`: A library for adding colored output to the terminal.

You can find these dependencies in `Cargo.toml`.

## Configuration

By default, the program uses a 4-thread setup for cracking the hash. You can modify this number in the source code by adjusting the `num_threads` variable to match your machine's capabilities for performance optimization.

