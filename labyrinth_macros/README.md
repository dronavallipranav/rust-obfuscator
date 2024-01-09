# labyrinth_macros

`labyrinth_macros` is a procedural macro crate designed to complement the `cryptify` super crate. It provides compile-time string and control flow obfuscation capabilities, aimed at enhancing the security and complexity of Rust code. Not meant to be used standalone, necessary obfuscation features are in the super crate `cryptify`

## Features

- **String Obfuscation**: Automatically encrypts string literals in your code at compile time, making them harder to read and understand.
- **Flow Obfuscation**: Introduces dummy loops and random variables into control flows, enhancing the overall obfuscation of the logic.

# License
labyrinth_macros is licensed under the MIT License - see the [LICENSE](https://github.com/dronavallipranav/rust-obfuscator/blob/main/LICENSE) file for details.