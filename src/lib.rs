#![allow(non_snake_case)]

mod Tterminal;
mod key;
use std::io::{self, Read};
use std::mem;
use std::os::unix::io::AsRawFd;

use crate::Tterminal::tterminal::{Ferminal, tcgetattr, tcsetattr};

pub fn ReadKeys() {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();
    unsafe {
        let mut tterminal: Ferminal = mem::zeroed();
        tcgetattr(fd, &mut tterminal);

        let original = tterminal;

        tterminal.c_lflag &= !(0x0002 | 0x0008);
        tcsetattr(fd, 0, &tterminal);

        let mut buf = [0u8; 1];
        stdin.lock().read_exact(&mut buf).unwrap();

        tcsetattr(fd, 0, &original);

        println!("Pressed: {}", buf[0]);
    }
}
