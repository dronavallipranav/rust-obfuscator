mod rename;
mod flow;
mod string;
mod obfuscate;
use crate::obfuscate::Obfuscator;

pub struct Config {
    pub rename_config: rename::RenameConfig,
    pub flow_config: flow::FlowConfig,
    pub string_config: string::StringConfig,
}

fn main() {
    let mut obfuscator = Obfuscator::new();
    let obfuscated_code = obfuscator.obfuscate("fn main() { println!(\"Hello, world!\"); }");
    println!("{}", obfuscated_code);
}
