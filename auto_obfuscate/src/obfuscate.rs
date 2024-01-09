use crate::string::{ StringObfuscator, StringConfig };
use crate::rename::{ VariableRenamer, RenameConfig };
use crate::flow::{ FlowObfuscator, FlowConfig };

#[derive(Clone)]
pub struct Config {
    pub rename_config: RenameConfig,
    pub flow_config: FlowConfig,
    pub string_config: StringConfig,
}

impl Config {
    pub fn default() -> Self {
        Self {
            rename_config: RenameConfig::default(),
            flow_config: FlowConfig::default(),
            string_config: StringConfig::default(),
        }
    }
}

pub struct Obfuscator {
    rename_obfuscator: VariableRenamer,
    flow_obfuscator: FlowObfuscator,
    string_obfuscator: StringObfuscator,
}

impl Obfuscator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            rename_obfuscator: VariableRenamer::new(RenameConfig::default()),
            flow_obfuscator: FlowObfuscator::new(FlowConfig::default()),
            string_obfuscator: StringObfuscator::new(StringConfig::default()),
        }
    }
    pub fn from_config(config: Config) -> Self {
        Self {
            rename_obfuscator: VariableRenamer::new(config.rename_config),
            flow_obfuscator: FlowObfuscator::new(config.flow_config),
            string_obfuscator: StringObfuscator::new(config.string_config),
        }
    }

    pub fn obfuscate(&mut self, code: &str) -> String {
        let mut result = code.to_string();
        if self.string_obfuscator.enabled {
            result = self.string_obfuscator.obfuscate_strings(&result);
        }
        if self.flow_obfuscator.enabled {
            result = self.flow_obfuscator.flow_obfuscate(&result);
        }
        if self.rename_obfuscator.enabled {
            result = self.rename_obfuscator.rename(&result);
        }
        result
    }
}
