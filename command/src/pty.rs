use std::{
    os::unix::prelude::{AsRawFd, RawFd},
    path::Path, io::Error,
};

use nix::{
    fcntl::{open, OFlag},
    pty::{grantpt, posix_openpt, ptsname_r, unlockpt},
    sys::{
        socket::{
            self, connect, sendmsg, AddressFamily, ControlMessage, MsgFlags, SockAddr, SockFlag,
            SockType, UnixAddr,
        },
        stat::Mode,
        uio::IoVec,
    },
    unistd::{close, dup2, setsid},
};

// use crate::core::common::{Error, ErrorType, Result};

pub struct PtySocket {
    pub socket_fd: RawFd,
}

impl PtySocket {
    pub fn new(console_socket_path: &String) -> PtySocket {
        let socket_fd = socket::socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::empty(),
            None,
        );

        PtySocket {
            socket_fd: socket_fd.as_raw_fd(),
        }
    }

    pub fn connect(socket_fd: i32, console_socket_path: &String) -> PtySocket {
        connect(
            socket_fd,
            &SockAddr::Unix(UnixAddr::new(console_socket_path.as_str()).unwrap()),
        );

        PtySocket {
            socket_fd: socket_fd.as_raw_fd(),
        } 
    }

    pub fn send(child_socket_fd: i32, msg: &str) {
        write(fd, msg_as_bytes().unwrap());
    }

    pub fn recv(child_socket_fd: i32) -> String {
        let mut buf = [0;1024];
        let num = read(fd, &mut buf).unwrap();
        let str = std::str::from_utf8(&buf[0..num]);
        str.trim().to_string()
    }

    pub fn close(&self) {
        close(self.socket_fd);
    }
}
