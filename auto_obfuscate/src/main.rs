mod rename;
mod flow;
mod string;
mod obfuscate;
use crate::obfuscate::{ Obfuscator, Config };
use clap::{ App, Arg };
use std::path::Path;
use std::fs;

fn main() {
    //default config
    let mut config = Config::default();
    let matches = App::new("Rust Automatic Obfuscator")
        .version("1.0")
        .author("Pranav Dronavalli")
        .about("Obfuscates Rust source code")
        .arg(
            Arg::with_name("path")
                .help("Path to the Rust file or directory")
                .required(true)
                .index(1)
        )
        .arg(Arg::with_name("no_string").long("no_string").help("Disable string obfuscation"))
        .arg(Arg::with_name("no_flow").long("no_flow").help("Disable control flow obfuscation"))
        .arg(
            Arg::with_name("disable_macro")
                .long("disable_macro")
                .help("disable macro and modify source directly for flow obfuscation")
        )
        .arg(Arg::with_name("var").long("var").help("Enable variable renaming")) 
        .get_matches();

    let path = matches.value_of("path").unwrap();

    //disable string obfuscation if the flag is set
    if matches.is_present("no_string") {
        config.string_config.enable_string_obfuscation = false;
    }

    //disable flow obfuscation if the flag is set
    if matches.is_present("no_flow") {
        config.flow_config.enable_flow_obfuscation = false;
    }

    //disable use of proc macro if the flag is set
    if matches.is_present("disable_macro") {
        config.flow_config.use_macro = false;
    }
    //enable variable renaming if the flag is set
    if matches.is_present("var") {
        config.rename_config.enable_rename_obfuscation = true;
    }
    //set upper bound for string literal encryption
    if let Some(percentage) = matches.value_of("p") {
        config.string_config.percentage = percentage.parse().unwrap_or(100);
    }

    process_path(&path, &config);
}

fn process_path(path_str: &str, config: &Config) {
    let path = Path::new(path_str);
    if path.is_dir() {
        process_directory(path, config);
    } else if path.is_file() {
        process_file(path, config);
    } else {
        eprintln!("Invalid path: {}", path_str);
    }
}
//process all files in directory
fn process_directory(dir_path: &Path, config: &Config) {
    for entry in fs::read_dir(dir_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            process_file(&path, config);
        }
    }
}
//read code from file
fn process_file(file_path: &Path, config: &Config) {
    if file_path.extension().unwrap_or_default() == "rs" {
        let code = fs::read_to_string(file_path).expect("Failed to read file");

        let mut obfuscator = Obfuscator::from_config(config.clone());
        let obfuscated_code = obfuscator.obfuscate(&code);

        //check if obfuscated code is valid Rust code
        let parse_result = syn::parse_file(&obfuscated_code);
        if parse_result.is_err() {
            eprintln!("Obfuscated code is not valid Rust code");
            return;
        }
        write_obfuscated_code(file_path, &obfuscated_code);
    }
}
//write file to obfuscated_code directory
fn write_obfuscated_code(original_path: &Path, obfuscated_code: &str) {
    let obfuscated_dir = Path::new("obfuscated_code");
    fs::create_dir_all(&obfuscated_dir).expect("Failed to create directory");

    let obfuscated_path = obfuscated_dir.join(original_path.file_name().unwrap());
    println!("Writing to {:?}", obfuscated_path);
    fs::write(obfuscated_path, obfuscated_code).expect("Failed to write obfuscated code");
}
