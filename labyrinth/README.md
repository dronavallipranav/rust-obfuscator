# labyrinth

`labyrinth` is a procedural macro crate for compile-time rust obfuscation. It provides the user with string encryption and compile-time determined flow obfuscation and random variables which survive compile-time optimization.


[rust-obfuscator](https://github.com/dronavallipranav/rust-obfuscator) - Check out this auto obfuscator tool for easier usage and integration
## Features

- **String Obfuscation**: Automatically encrypts string literals in your code at compile time, making them harder to read and understand.
- **Flow Obfuscation**: Introduces dummy loops and random variables into control flows, enhancing the overall obfuscation of the logic.

# Installation

Add `labyrinth` to your `Cargo.toml` as a dependency:

```toml
[dependencies]
labyrinth = "1.0.0"
```

# Usage

## Bring macro into scope
```
use labyrinth;

fn main(){
    println!(labyrinth::encrypt_string("Hello, World!"));
}
```
## Output
```
Hello World!
```
## Example of expanded Flow_Stmt!

```
    {
        let _is_dummy_145 = true;
        let _dummy_upper_bound = 100;
        let _dummy_increment = 1i32;
        let mut _dummy_counter = 10i32;
        let _extra_dummy_var = 2i32;
        loop {
            if _dummy_counter > _dummy_upper_bound {
                break;
            }
            unsafe {
                std::ptr::write_volatile(
                    &mut _dummy_counter,
                    _dummy_counter + _dummy_increment,
                );
            }
        }
    };
    match (&1, &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };

```

# License
labyrinth is licensed under the MIT License - see the [LICENSE](https://github.com/dronavallipranav/rust-obfuscator/blob/main/LICENSE) file for details.
