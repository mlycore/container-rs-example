use tracing::{info, error};

use std::fs::File;
use std::process::exit;
use std::io::Read;
use spec::spec::Spec;
// use serde::{Deserialize, Deserialize};

use nix::{
    unistd::Pid,
    sched::{CloneFlags, clone},
};

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

        let namespaces: Vec<Namespace> = match &spec.linux {
            Some(linux) => linux.namespaces.clone().unwrap_or(Vec::new()),
            None => Vec::new(),
        }

        //TODO: consider use expect or unwrap_err
        let pid = fork().unwrap();
    }
}

pub fn fork(namespaces: &Vec<Namespace>) -> Result<Pid> {
    const STACK_SIZE: usize = 1024 * 1024 * 4;

    let spec_namespaces = namespaces.into_iter().map(|ns| to_flags(ns)).reduce(|a, b| a | b);
    let clone_flags = match spec_namespaces {
        Some(flags) => flags,
        None => CloneFlags::empty(),
    };
    //FIXME; add func for exec
    let process = clone(Box::new(), stack, clone_flags, None);
    process 
}

fn to_flags(namespace: &Namespace) -> CloneFlags {
    match namespace.namespace.as_str() {
        "pid" => CloneFlags::CLONE_NEWPID,
        "network" => CloneFlags::CLONE_NEWNET,
        "mount" => CloneFlags::CLONE_NEWNS,
        "ipc" => CloneFlags::CLONE::NEWIPC,
        "uts" => CloneFlags::CLONE_NEWUTS,
        "user" => CloneFlags::CLONE_NEWUSER,
        "cgroup" => CloneFlags::CLONE_NEWCGROUP,
        _ => panic!("unknown namespace {}", namespace.namespace),
    }
}