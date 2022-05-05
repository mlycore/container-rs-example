use clap::{Command, Arg};
use tracing::error;

use crate::create::CreateCommand;

pub fn new_command() {
    let matches = Command::new("container-rs")
    .version("0.1.0")
    .arg(Arg::new("log-level")
                .long("log-level")
                .takes_value(true)
                .help("log level (e.g. error, warn, info)")
    )
    .subcommand(Command::new("create")
            .arg(
                Arg::new("container-id")
                    .takes_value(true)
                    .help("the id of container")
            )
            .arg(
                Arg::new("oci-config")        
                    .long("oci-config")
                    .short('c')
                    .takes_value(true)
                    .required(true)
                    .help("oci-config config.json directory")
            )
    )
    .get_matches();

    match matches.subcommand() {
        Some(cmd) => {
            match cmd.0 {
                "create" => {
                    let args = cmd.1;
                    let cmd_create = CreateCommand {
                        id: args.value_of("container-id").expect("container id is required").to_string(),
                        oci_config: args.value_of("oci-config").expect("oci config is required").to_string(),
                    };
                    cmd_create.exec(); 
                }    
                _ => {
                    error!("Unknown subcommand") 
                }
            }

        }
        None => {
            error!("Unknown command") 
        }
    }


}
