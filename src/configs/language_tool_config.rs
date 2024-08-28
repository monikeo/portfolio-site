use serde::Deserialize;
use std::fs;
use lazy_static::lazy_static;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct Skill {
    pub name: String,
    pub skill_icon: String
}


#[derive(Debug, Deserialize, Clone)]
pub struct SkillsConfig{ 
    pub language: Vec<Skill>,
    pub tool: Vec<Skill>
}

lazy_static! {
    pub static ref SkillLanguageConfig: Arc<SkillsConfig> = {
        let path = "../../../../data_config/language_tool.config.toml";
        let content = fs::read_to_string(path).expect("failed to read file");
        let config = toml::from_str(&content).expect("failed to parsing toml format");
        Arc::new(config)
    };
}

pub fn get_config() -> Arc<SkillsConfig> {
    Arc::clone(&SkillLanguageConfig)
}

