use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // #[serde(default = "default_web")]
    pub web: Vec<PortConfig>,
    // #[serde(default = "default_app")]
    pub app: Vec<PortConfig>,
    // #[serde(default = "default_db")]
    pub database: Vec<PortConfig>,

    pub jumpserver: Vec<PortConfig>,

    pub domain: Vec<PortConfig>,

    pub loadbalancer: Vec<PortConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortConfig {
    pub port: String,
    pub protocol: String,
}

pub fn default_web() -> Vec<PortConfig> {
    vec![
        PortConfig {
            port: "80".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
        port: "443".to_string(),
        protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "135".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "135".to_string(),
            protocol: "udp".to_string(),
        },
        PortConfig {
            port: "445".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "445".to_string(),
            protocol: "udp".to_string(),
        },
        PortConfig {
            port: "3389".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "3389".to_string(),
            protocol: "udp".to_string(),
        },
        PortConfig {
            port: "5985".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "5986".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "7770".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "7771".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "7780".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "7781".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "49152".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "49152".to_string(),
            protocol: "udp".to_string(),
        },
    ]
}

pub fn default_app() -> Vec<PortConfig> {
    vec![
        PortConfig {
            port: "135".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "135".to_string(),
            protocol: "udp".to_string(),
        },
        PortConfig {
            port: "445".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "445".to_string(),
            protocol: "udp".to_string(),
        },
        PortConfig {
            port: "3389".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "3389".to_string(),
            protocol: "udp".to_string(),
        },
        PortConfig {
            port: "5985".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "5986".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "2109".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "45054".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "49152".to_string(),
            protocol: "tcp".to_string(),
        },
        PortConfig {
            port: "49152".to_string(),
            protocol: "udp".to_string(),
        },
    ]
}

pub fn default_database() -> Vec<PortConfig> {
    vec![PortConfig {
        port: "5432".to_string(),
        protocol: "tcp".to_string(),
    }]
}

pub fn default_jumpserver() -> Vec<PortConfig> {
    vec![
        PortConfig {
            port: "22".to_string(),
            protocol: "tcp".to_string(),
        }
    ]
}

pub fn default_domain() -> Vec<PortConfig> {
    vec![
        PortConfig {
            port: "22".to_string(),
            protocol: "tcp".to_string(),
        }
    ]
}

pub fn default_loadbalancer() -> Vec<PortConfig> {
    vec![
        PortConfig {
            port: "443".to_string(),
            protocol: "tcp".to_string(),
        }
    ]
}
