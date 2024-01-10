# Rust-Obfuscator

`rust-obfuscator` is a set of tools designed to automatically obfuscate Rust source code by automatically inserting procedural macros or by (optionally) providing the obfuscation in the source code directly. For more fine-grained obfuscation a procedural macro library [cryptify](https://crates.io/crates/cryptify) is also provided.

## Currently Supported
1. String literal encryption
2. control-flow obfuscation
3. control-flow obfuscation (source code)
4. variable renaming (source code)

## Features

- **String Encryption**: Automatically encrypts string literals assigned to local variables at compile time.
    - Can also be used for formatted strings, but currently requires manual placement
    ```rs
        println!("{}", cryptify::encrypt_string!("hello!"));
    ```
- **Control Flow Obfuscation**: Introduces compile-dummy dummy loops and random variables.
    - Note: for truly random control flow and variables, you can disable the insertion of the flow_macro using the **disable_macro** flag, but this will directly affect the source code.
- **Customizable Obfuscation**: Offers flexibility to enable or disable specific obfuscation features based on your requirements.
- **Variable Renaming**: Obfuscation of the source code directly, if you'd like to ship the code or just want to make your code look worse.

## Installation

Add `cryptify` to your `Cargo.toml` as a dependency:

```toml
[dependencies]
cryptify = "3.0.8"
```

To install `rust-obfuscator`, clone the repository and build the tool using Cargo from the root:
```
cargo build --release --bin rust-obfuscator
```
The binary can then be found under /target/release, you can copy it to the root of the project like such
```
cp ./target/release/rust-obfuscator .
```

# Usage
Set the **CRYPTIFY_KEY** environment variable for custom encryption otherwise it defaults to defined fixed key
- Add to source code you'd like to modify
```rs
use cryptify;
```
The binary can be used on either a file or a directory. If provided with a directory it will only modify rust source files within that directory not any subdirectories
```sh
./rust-obfuscator path/to/your_project <Options>
```
- All Obfuscated code will be under the **obfuscated_code** directory that is created from the directory the tool was run.
- **Recommended to use a Rust Formatter with the obfuscated code as syn naturally modifies the structure and it will be written to the file as one line**

## Option Flags
- --no_string: Disables string obfuscation.
- --no_flow: Disables control flow obfuscation.
- --disable_macro: Uses direct source manipulation for flow obfuscation instead of procedural macros.
- --var: Enables variable renaming source code obfuscation.

### Example usage with flag
```sh
rust-obfuscator path/to/your_project --no_flow 
```
(disables flow obfuscation)

# Input
-running the tool with no config
```rs
use cryptify;
mod word_counter;
use std::env;
use std::fs;
use word_counter::count_words;
fn main() {
    let b = "Hello World";
    println!("{}", b);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Could not read file");
    let word_counts = count_words(&content);
    for (word, count) in word_counts.iter() {
        println!("{}: {}", word, count);
    }
}

fn dummy() {
    let a = 1;
    let b = 2;
    let c = a + b;
    println!("{}", c);
}

fn calc_sum(a: i32, b: i32) -> i32 {
    cryptify::flow_stmt!();
    let c = a + b;
    c
}

fn helloooo(){
    println!("hi");
}

```
# Output
```rs
use cryptify;
mod word_counter;
use std::env;
use std::fs;
use word_counter::count_words;
fn main() {
    cryptify::flow_stmt!();
    let b = cryptify::encrypt_string!("Hello World");
    println!("{}", b);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Could not read file");
    let word_counts = count_words(&content);
    for (word, count) in word_counts.iter() {
        println!("{}: {}", word, count);
    }
}
fn dummy() {
    cryptify::flow_stmt!();
    let a = 1;
    let b = 2;
    let c = a + b;
    println!("{}", c);
}
fn calc_sum(a: i32, b: i32) -> i32 {
    cryptify::flow_stmt!();
    let c = a + b;
    c
}
fn helloooo() {
    println!("hi");
}
```
# License
rust-obfuscator is licensed under the MIT License - see the [LICENSE](https://github.com/dronavallipranav/rust-obfuscator/blob/main/LICENSE) file for details.
