use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Skill {
    name: String,
    skill_icon: String
}

impl Skill {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn skill_icon(&self) -> &str {
        &self.skill_icon
    }
}

#[derive(Debug, Deserialize)]
pub struct SkillsConfig{ 
    language: Vec<Skill>,
    tool: Vec<Skill>
}

impl SkillsConfig {
    pub fn language(&self) -> &Vec<Skill> {
        self.language.as_ref()
    }
    pub fn tool(&self) -> &Vec<Skill> {
        self.tool.as_ref()
    }
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(path)?;
        let config: SkillsConfig = toml::from_str(&config_content)?;
        Ok(config)
    }
}
