use crate::string::{ StringObfuscator, StringConfig };
use crate::rename::{ VariableRenamer, RenameConfig };
use crate::flow::{ FlowObfuscator, FlowConfig };

pub struct Obfuscator {
    rename_obfuscator: VariableRenamer,
    flow_obfuscator: FlowObfuscator,
    string_obfuscator: StringObfuscator,
}

impl Obfuscator {
    pub fn new() -> Self {
        Self {
            rename_obfuscator: VariableRenamer::new(RenameConfig::default()),
            flow_obfuscator: FlowObfuscator::new(FlowConfig::default()),
            string_obfuscator: StringObfuscator::new(StringConfig::default()),
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
