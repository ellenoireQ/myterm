#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ferminal {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

unsafe extern "C" {
    pub unsafe fn tcgetattr(fd: i32, tterminal: *mut Ferminal) -> i32;
    pub unsafe fn tcsetattr(fd: i32, optional_actions: i32, tterminal: *const Ferminal) -> i32;
}
