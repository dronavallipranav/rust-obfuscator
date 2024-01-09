# Rust-Obfuscator

`rust-obfuscator` is a set of tools designed to automatically obfuscate Rust source code by automatically inserting procedural macros or by (optionally) providing the obfuscation in the source code directly. For more fine-grained obfuscation a procedural macro library [cryptify](https://crates.io/crates/cryptify) is also provided.

## Currently Supported
1. String literal encryption
2. control-flow obfuscation
3. control-flow obfuscation (source code)
4. variable renaming (source code)

## Features

- **String Encryption**: Encrypts string literals in your code at compile time.
- **Control Flow Obfuscation**: Introduces compile-dummy dummy loops and random variables into control flows complicating the logic and structure of the code.
- Note: for truly random control flow and variables, you can disable the insertion of the flow_macro using the **disable_macro** flag, but this will directly affect the source code.
- **Customizable Obfuscation**: Offers flexibility to enable or disable specific obfuscation features based on your requirements.
- **Variable Renaming**: Obfuscation of the source code directly, if you'd like to ship the code or just want to make your code look worse.

## Installation

To install `rust-obfuscator`, clone the repository and build the tool using Cargo from the root:
```
cargo build --release --bin rust-obfuscator
```
The binary can then be found under /target/release, you can copy it to the root of the project like such
```
cp ./target/release/rust-obfuscator .
```

# Usage
The binary can be used on either a file or a directory. If provided with a directory it will only modify rust source files within that directory not any subdirectories
```
rust-obfuscator path/to/your_project <Options>
```
- All Obfuscated code will be under the **obfuscated_code** directory that is created from the directory the tool was run.

## Option Flags
- --no_string: Disables string obfuscation.
- --no_flow: Disables control flow obfuscation.
- --disable_macro: Uses direct source manipulation for flow obfuscation instead of procedural macros.
- --var: Enables variable renaming source code obfuscation.

### Example usage with flag
```
rust-obfuscator path/to/your_project --no_flow 
```
(disables flow obfuscation)

# License
labyrinth_macros is licensed under the MIT License - see the [LICENSE](https://github.com/dronavallipranav/rust-obfuscator/blob/main/LICENSE) file for details.