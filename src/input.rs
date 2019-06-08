use libc::{read, open, input_event, O_RDONLY, close};
use std::ffi::CString;

pub struct Keyboard {
    fd: i32,
    ev: input_event
}

#[derive(Debug)]
pub struct KeyEvent {
    code: u16,
    value: i32
}

impl Keyboard {
    pub fn new<T>(v: T) -> Result<Keyboard, ()> where T: Into<Vec<u8>> {
        let ev: input_event = unsafe { std::mem::MaybeUninit::uninit().assume_init() };// { code: 1, time: libc::timeval { tv_sec: 0, tv_usec: 0 }, type_: 1, value: 1 };

        let input_file = CString::new(v).unwrap();

        let fd = unsafe { open(input_file.as_ptr() as *const i8, O_RDONLY) };
        if fd < 0 {
            Err(())
        } else {
            Ok(Keyboard { fd, ev })
        }
    }

    pub fn key(&mut self) -> KeyEvent {
        self.ev.type_ = 2;
        loop {
            unsafe { read(self.fd, &mut self.ev as *mut _ as *mut std::ffi::c_void, std::mem::size_of::<input_event>()) };
            if self.ev.type_ == 0x01 { // is key event: input-event-keycode
                return KeyEvent { code: self.ev.code, value: self.ev.value }
            }
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        unsafe { close(self.fd) };
    }
}
