// r2qd_generate_a_auto.rs

use std::fs;
use std::path::Path;
use regex::Regex;

struct AutomationScript {
    filename: String,
    content: String,
    tags: Vec<String>,
}

impl AutomationScript {
    fn new(filename: &str, content: &str) -> AutomationScript {
        let re = Regex::new(r"#\s*tags\s*:\s*(.*)").unwrap();
        let caps = re.captures(content);
        let tags = match caps {
            Some(c) => c.get(1).unwrap().as_str().split(",").map(|s| s.trim().to_string()).collect(),
            None => Vec::new(),
        };
        AutomationScript {
            filename: filename.to_string(),
            content: content.to_string(),
            tags,
        }
    }
}

struct AutomationScriptAnalyzer {
    scripts: Vec<AutomationScript>,
}

impl AutomationScriptAnalyzer {
    fn new() -> AutomationScriptAnalyzer {
        AutomationScriptAnalyzer { scripts: Vec::new() }
    }

    fn load_scripts(&mut self, path: &str) {
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() && path.extension().unwrap() == "auto" {
                let content = fs::read_to_string(path).unwrap();
                let script = AutomationScript::new(path.file_name().unwrap().to_str().unwrap(), &content);
                self.scripts.push(script);
            }
        }
    }

    fn analyze_scripts(&self) {
        for script in &self.scripts {
            println!("Analyzing script: {}", script.filename);
            for tag in &script.tags {
                println!("  Tag: {}", tag);
            }
        }
    }
}

fn main() {
    let mut analyzer = AutomationScriptAnalyzer::new();
    analyzer.load_scripts("/path/to/automation/scripts");
    analyzer.analyze_scripts();
}