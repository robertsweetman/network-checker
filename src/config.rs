use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // #[serde(default = "default_web")]
    pub web: Vec<PortConfig>,
    // #[serde(default = "default_app")]
    pub app: Vec<PortConfig>,
    // #[serde(default = "default_db")]
    pub db: Vec<PortConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortConfig {
    pub port: String,
    pub protocol: String,
}

pub fn default_web() -> Vec<PortConfig> {
    vec![
        PortConfig {
            port: "90".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
        port: "443".to_string(),
        protocol: "tcp".to_string(),
        },
    ]
}

pub fn default_app() -> Vec<PortConfig> {
    vec![PortConfig {
        port: "9000".to_string(),
        protocol: "tcp".to_string(),
    }]
}

pub fn default_db() -> Vec<PortConfig> {
    vec![PortConfig {
        port: "5432".to_string(),
        protocol: "tcp".to_string(),
    }]
}

