use tracing::{info, error};

use std::fs::File;
use std::process::exit;
use std::io::Read;
use spec::spec::Spec;
// use serde::{Deserialize, Deserialize};

pub struct CreateCommand {
    pub id: String,
    pub oci_config: String,
}

impl CreateCommand {
    pub fn exec(&self) {
        info!("create command: {:?}, {:?}", self.id, self.oci_config);

        let mut file = match File::open(&self.oci_config) {
            Err(e) => {
                error!("open bundle error: {:?}", e);
                exit(-1);
            }
            Ok(file) => file,
        };

        let mut spec_json = String::new();
        // file.read_to_string(&mut config_str).unwrap();
        // config = toml::from_str(&config_str).unwrap();

        file.read_to_string(&mut spec_json).unwrap();
        let spec: Spec = serde_json::from_str(&spec_json).unwrap();
        info!("spec_json: {:?}", spec);    
    }
}

