use tracing::{info, error};

use std::fs::File;
use std::process::exit;
use std::io::Read;
// use std::error::Error;
use std::fmt::Error;
use std::result::Result;
use std::path::Path;
use std::ffi::CString;
use spec::spec::{Spec, Namespace};
// use serde::{Deserialize, Deserialize};

use nix::{
    unistd::{Pid, sethostname, setuid, setgid, Uid, Gid, chdir, execvp},
    sched::{CloneFlags, clone},
    fcntl::{open, OFlag},
    mount::{mount, MntFlags, MsFlags},
    sys::stat::Mode,
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
        };

        //TODO: consider use expect or unwrap_err
        let pid = fork(&spec, &namespaces).unwrap();
        info!("pid: {}", pid);
    }
}

pub fn fork(spec: &Spec, namespaces: &Vec<Namespace>) -> Result<Pid, nix::Error> {
    const STACK_SIZE: usize = 1024 * 1024 * 4;
    let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

    let spec_namespaces = namespaces.into_iter().map(|ns| to_flags(ns)).reduce(|a, b| a | b);
    let clone_flags = match spec_namespaces {
        Some(flags) => flags,
        None => CloneFlags::empty(),
    };

    let func = || {
        if let Some(linux) = &spec.linux {
            if let Some(namespaces) = &linux.namespaces {
                for ns in namespaces {
                    if let Some(path) = &ns.path {
                        let fd = match open(path.as_str(), OFlag::empty(), Mode::empty()) {
                            Ok(fd) => fd, 
                            Err(err) => {
                                error!("open file error: {}", err);
                                exit(-1);
                            }
                        };
                    }
                }
            }
        }

        let rootfs = Path::new(&spec.root.path); 
        if let Err(err) = mount_rootfs(&rootfs) {
            error!("mount rootfs error: {}", err);
            exit(-1);
        }

        if let Some(hostname) = &spec.hostname {
            sethostname(hostname).unwrap();
        }

        if let Some(process) = &spec.process {
            let cmd = &process.args.as_ref().unwrap()[0]; 
            let args: Vec<CString> = spec.process.as_ref().unwrap().args.as_ref().unwrap().iter().map(|a| CString::new(a.to_string()).unwrap_or_default()).collect();
            let exec = CString::new(cmd.as_bytes()).unwrap();

            if let Some(envs) = &process.env {
                for env in envs {
                    if let Some((key, value)) = env.split_once("=") {
                        std::env::set_var(key, value);
                    }
                } 
            }

            if let Some(user) = &process.user {
                setuid(Uid::from_raw(user.uid as u32)).unwrap();
                setgid(Gid::from_raw(user.gid as u32)).unwrap();
            }

            chdir(Path::new(&process.cwd)).unwrap();

            match execvp(&exec, &args) {
                Ok(_) => (),
                Err(err) => {
                    error!("execvp error: {}", err);
                    exit(-1);
                }
            }
        }

        0
    };

     
    //FIXME; add func for exec
    let process = clone(Box::new(func), stack, clone_flags, None);
    process 
}

fn to_flags(namespace: &Namespace) -> CloneFlags {
    match namespace.namespace.as_str() {
        "pid" => CloneFlags::CLONE_NEWPID,
        "network" => CloneFlags::CLONE_NEWNET,
        "mount" => CloneFlags::CLONE_NEWNS,
        "ipc" => CloneFlags::CLONE_NEWIPC,
        "uts" => CloneFlags::CLONE_NEWUTS,
        "user" => CloneFlags::CLONE_NEWUSER,
        "cgroup" => CloneFlags::CLONE_NEWCGROUP,
        _ => panic!("unknown namespace {}", namespace.namespace),
    }
}

fn mount_rootfs(rootfs: &Path) -> Result<(), nix::Error> {
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE | MsFlags::MS_REC,
        None::<&str>,
    )?;

    mount::<Path, Path, str, str>(
        Some(&rootfs),
        &rootfs,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    )?;

    Ok(())
}