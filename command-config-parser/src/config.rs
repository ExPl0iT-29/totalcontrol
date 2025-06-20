// config.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub executable: String,
    pub args: Vec<String>,
    pub working_dir: Option<String>,
    pub env_vars: Option<HashMap<String, String>>,
    pub aliases: Option<Vec<String>>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandConfig {
    pub version: String,
    pub commands: Vec<Command>,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    ParseError(String),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "Configuration file not found"),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::IoError(err) => write!(f, "IO error: {}", err),
            ConfigError::JsonError(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::JsonError(err)
    }
}

pub struct CommandRegistry {
    commands: Vec<Command>,
    name_map: HashMap<String, usize>,
    alias_map: HashMap<String, usize>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            name_map: HashMap::new(),
            alias_map: HashMap::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        if !path.as_ref().exists() {
            return Err(ConfigError::FileNotFound);
        }

        let content = fs::read_to_string(path)?;
        let config: CommandConfig = serde_json::from_str(&content)?;
        
        let mut registry = Self::new();
        registry.load_commands(config.commands)?;
        
        Ok(registry)
    }

    pub fn load_from_default() -> Result<Self, ConfigError> {
        Self::load_from_file("commands.json")
    }

    fn load_commands(&mut self, commands: Vec<Command>) -> Result<(), ConfigError> {
        self.commands.clear();
        self.name_map.clear();
        self.alias_map.clear();

        for (index, command) in commands.into_iter().enumerate() {
            // Check for duplicate command names
            if self.name_map.contains_key(&command.name) {
                return Err(ConfigError::ParseError(format!(
                    "Duplicate command name: {}", command.name
                )));
            }

            // Register command name
            self.name_map.insert(command.name.clone(), index);

            // Register aliases
            if let Some(ref aliases) = command.aliases {
                for alias in aliases {
                    if self.alias_map.contains_key(alias) || self.name_map.contains_key(alias) {
                        return Err(ConfigError::ParseError(format!(
                            "Duplicate alias or command name: {}", alias
                        )));
                    }
                    self.alias_map.insert(alias.clone(), index);
                }
            }

            self.commands.push(command);
        }

        Ok(())
    }

    pub fn get_command(&self, name: &str) -> Option<&Command> {
        // First try direct name lookup
        if let Some(&index) = self.name_map.get(name) {
            return self.commands.get(index);
        }
        
        // Then try alias lookup
        if let Some(&index) = self.alias_map.get(name) {
            return self.commands.get(index);
        }

        None
    }

    pub fn get_all_commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn get_command_names(&self) -> Vec<&String> {
        self.name_map.keys().collect()
    }

    pub fn get_completions(&self, prefix: &str) -> Vec<String> {
        let mut completions = Vec::new();
        
        // Add matching command names
        for name in self.name_map.keys() {
            if name.starts_with(prefix) {
                completions.push(name.clone());
            }
        }
        
        // Add matching aliases
        for alias in self.alias_map.keys() {
            if alias.starts_with(prefix) {
                completions.push(alias.clone());
            }
        }
        
        completions.sort();
        completions
    }

    pub fn get_commands_by_category(&self, category: &str) -> Vec<&Command> {
        self.commands
            .iter()
            .filter(|cmd| {
                cmd.category
                    .as_ref()
                    .map_or(false, |cat| cat == category)
            })
            .collect()
    }

    pub fn get_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self.commands
            .iter()
            .filter_map(|cmd| cmd.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        categories.sort();
        categories
    }
}

// Helper function to create a sample configuration file
pub fn create_sample_config<P: AsRef<Path>>(path: P) -> Result<(), ConfigError> {
    let sample_config = CommandConfig {
        version: "1.0".to_string(),
        commands: vec![
            Command {
                name: "build".to_string(),
                description: "Build the project".to_string(),
                executable: "cargo".to_string(),
                args: vec!["build".to_string()],
                working_dir: None,
                env_vars: None,
                aliases: Some(vec!["b".to_string()]),
                category: Some("development".to_string()),
            },
            Command {
                name: "test".to_string(),
                description: "Run tests".to_string(),
                executable: "cargo".to_string(),
                args: vec!["test".to_string()],
                working_dir: None,
                env_vars: None,
                aliases: Some(vec!["t".to_string()]),
                category: Some("development".to_string()),
            },
            Command {
                name: "deploy".to_string(),
                description: "Deploy to production".to_string(),
                executable: "bash".to_string(),
                args: vec!["-c".to_string(), "echo 'Deploying...'".to_string()],
                working_dir: Some("/opt/app".to_string()),
                env_vars: Some({
                    let mut env = HashMap::new();
                    env.insert("ENVIRONMENT".to_string(), "production".to_string());
                    env
                }),
                aliases: Some(vec!["d".to_string(), "prod".to_string()]),
                category: Some("deployment".to_string()),
            },
        ],
    };

    let json = serde_json::to_string_pretty(&sample_config)?;
    fs::write(path, json)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        create_sample_config(temp_file.path()).unwrap();
        
        let registry = CommandRegistry::load_from_file(temp_file.path()).unwrap();
        
        assert_eq!(registry.get_all_commands().len(), 3);
        assert!(registry.get_command("build").is_some());
        assert!(registry.get_command("b").is_some()); // alias
        assert!(registry.get_command("nonexistent").is_none());
    }

    #[test]
    fn test_completions() {
        let mut temp_file = NamedTempFile::new().unwrap();
        create_sample_config(temp_file.path()).unwrap();
        
        let registry = CommandRegistry::load_from_file(temp_file.path()).unwrap();
        
        let completions = registry.get_completions("b");
        assert!(completions.contains(&"build".to_string()));
        assert!(completions.contains(&"b".to_string()));
    }

    #[test]
    fn test_categories() {
        let mut temp_file = NamedTempFile::new().unwrap();
        create_sample_config(temp_file.path()).unwrap();
        
        let registry = CommandRegistry::load_from_file(temp_file.path()).unwrap();
        
        let categories = registry.get_categories();
        assert!(categories.contains(&"development".to_string()));
        assert!(categories.contains(&"deployment".to_string()));
        
        let dev_commands = registry.get_commands_by_category("development");
        assert_eq!(dev_commands.len(), 2);
    }
}