use std::fs::File;
use config::Config;
use std::io::{self, Write, Read};
use clap::{Arg, App};
use config::{default_web, default_app, default_db};

mod check_tcp;
mod check_udp;
mod config;

fn main() {
    let matches = App::new("Connectivity Analyzer")
        .subcommand(App::new("config")
            .about("Uses a config file")
            .arg(Arg::with_name("HOST")
                .help("The host to connect to")
                .required(true)
                .index(1))
            .arg(Arg::with_name("TYPE")
                .help("The type of the host (web, app, or db)")
                .required(true)
                .index(2))
            .arg(Arg::with_name("FILE")
                .help("The config file to use")
                .required(true)
                .index(3)))
        .subcommand(App::new("default")
            .about("Uses the default config")
            .arg(Arg::with_name("HOST")
                .help("The host to connect to")
                .required(true)
                .index(1))
            .arg(Arg::with_name("TYPE")
                .help("The type of the host (web, app, or db)")
                .required(true)
                .index(2)))
        .subcommand(App::new("manual")
            .about("Manual input")
            .arg(Arg::with_name("HOST")
                .help("The host to connect to")
                .required(true)
                .index(1))
            .arg(Arg::with_name("PORT")
                .help("The port to connect to")
                .required(true)
                .index(2))
            .arg(Arg::with_name("PROTOCOL")
                .help("The protocol to use (tcp or udp)")
                .required(true)
                .index(3)))
        .get_matches();

    match matches.subcommand() {
        Some(("config", config_matches)) => {
            let host_type = config_matches.value_of("TYPE").expect("Type is required when using a config file");
            let host = config_matches.value_of("HOST").expect("Host is required when using a config file");
            let config_file = config_matches.value_of("FILE").expect("Config file is required when using a config file");
        
            let mut file = File::open(config_file).expect("Unable to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Unable to read file");
        
            let data: Config = serde_json::from_str(&contents).expect("Unable to parse JSON");
        
            let configs = match host_type {
                "web" => &data.web_config,
                "app" => &data.app_config,
                "db" => &data.db_config,
                _ => unreachable!(),
            };
            
            for item in configs {
                let port = &item.port;
                let protocol = &item.protocol;
        
                check_connection(protocol, host, port);
            }
        },

        Some(("manual", manual_matches)) => {
            let host = manual_matches.value_of("HOST").expect("Host is required when not using a config file");
            let port = manual_matches.value_of("PORT").expect("Port is required when not using a config file");
            let protocol = manual_matches.value_of("PROTOCOL").expect("Protocol is required when not using a config file");

            check_connection(protocol, host, port);
        },
        Some(("default", default_matches)) => {
            let host_type = default_matches.value_of("TYPE").expect("Type is required when using the default config");
            let host = default_matches.value_of("HOST").expect("Host is required when using the default config");
    
            let data = Some(Config {
                web_config: default_web(),
                app_config: default_app(),
                db_config: default_db(),
            });
    
            if let Some(config) = data {
                let configs = match host_type {
                    "web" => &config.web_config,
                    "app" => &config.app_config,
                    "db" => &config.db_config,
                    _ => unreachable!(),
                };
                
                for item in configs {
                    let port = &item.port;
                    let protocol = &item.protocol;
    
                    check_connection(protocol, host, port);
                }
            }
        },
        _ => unreachable!(),
    }

}

fn check_connection(protocol: &str, host: &str, port: &str) {
    match protocol {
        "tcp" => {
            check_tcp::check_tcp_connection(host, port);
        }
        "udp" => {
            check_udp::check_udp_connection(host, port);
        }    
        _ => {
            writeln!(io::stderr(), "Unknown protocol: {}", protocol).unwrap();
        }
    }
}