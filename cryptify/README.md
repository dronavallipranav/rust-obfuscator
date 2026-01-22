# cryptify

`cryptify` is a procedural macro crate for compile-time rust obfuscation. It provides the user with string encryption and compile-time determined flow obfuscation and random variables which survive compile-time optimization.


[rust-obfuscator](https://github.com/dronavallipranav/rust-obfuscator) - Check out this auto obfuscator tool for easier usage and integration
## Features

- **String Obfuscation**: Automatically encrypts string literals in your code at compile time, making them harder to read and understand.
- **Flow Obfuscation**: Introduces dummy loops and random variables into control flows, enhancing the overall obfuscation of the logic.

# Usage

## Bring macro into scope
```rs
use cryptify;

fn main(){
    let decrypted = cryptify::encrypt_string!("Hello, World!");
    println!(decrypted);
    println!("{}", cryptify::encrypt_string!("formatted!"));
}
```

Set the **CRYPTIFY_KEY** environment variable for custom encryption otherwise it defaults to defined fixed key

## Output
```
Hello World!
formatted!
```
## Example of expanded Flow_Stmt!

```rs
{
    {
        let _is_dummy_145 = true;
        let mut _dummy_counter = std::hint::black_box(4i32 as i32);
        let _dummy_increment = std::hint::black_box(1i32 as i32);
        let _dummy_upper_bound = std::hint::black_box(53i32 as i32);
        loop {
            if std::hint::black_box(_dummy_counter)
                > std::hint::black_box(_dummy_upper_bound)
            {
                break;
            }
            _dummy_counter = std::hint::black_box(
                std::hint::black_box(_dummy_counter)
                    + std::hint::black_box(_dummy_increment),
            );
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
}

```

# License
cryptify is licensed under the MIT License - see the [LICENSE](https://github.com/dronavallipranav/rust-obfuscator/blob/main/LICENSE) file for details.
