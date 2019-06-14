use x11::xlib::{Display, XOpenDisplay, XCloseDisplay, XQueryExtension, XDefaultRootWindow, XSync, XEvent, XNextEvent, XGetEventData, GenericEvent, XkbKeycodeToKeysym, NoSymbol, XKeysymToString};
use x11::xinput2::{XIEventMask, XIAllMasterDevices, XI_LASTEVENT, XISetMask, XI_RawKeyPress, XI_RawKeyRelease, XISelectEvents, XIRawEvent};

pub struct XDisplay {
    disp: *mut Display
}

impl XDisplay {
    pub fn new() -> XDisplay {
        let display = std::ffi::CString::new(":0").expect("Cannot create display name");
        let disp = unsafe { XOpenDisplay(display.as_ptr()) };
        if disp.is_null() {
            panic!("Cannot open display")
        }
        XDisplay { disp }
    }
}

impl Drop for XDisplay {
    fn drop(&mut self) {
        unsafe { XCloseDisplay(self.disp) };
    }
}

pub struct XKeyboard {
    disp: XDisplay,
    xi_opcode: i32,
}

impl XKeyboard {
    pub fn new(disp: XDisplay) -> XKeyboard {
        let xi_opcode = {
            let extension_name = std::ffi::CString::new("XInputExtension").expect("Cannot create extension name");
            let mut xi_opcode = 0;
            let mut query_event = 0;
            let mut query_error = 0;
            if 0 == unsafe { XQueryExtension(disp.disp, extension_name.as_ptr(), &mut xi_opcode, &mut query_event, &mut query_error) } {
                // XXX Test version >=2
                panic!("X Input extension not available")
            }
            xi_opcode
        };

        { // Register event
            let root = unsafe { XDefaultRootWindow(disp.disp) };

            let mut m = unsafe { ::std::mem::MaybeUninit::<XIEventMask>::zeroed().assume_init() };
            m.deviceid = XIAllMasterDevices;
            m.mask_len = ((XI_LASTEVENT) >> 3) + 1;
            let mut v = vec![0; m.mask_len as usize];

            XISetMask(&mut v, XI_RawKeyPress);
            XISetMask(&mut v, XI_RawKeyRelease);
            m.mask = v.as_mut_ptr();

            unsafe {
                XISelectEvents(disp.disp, root, &mut m, 1);
                XSync(disp.disp, false as i32)
            };
        }
        XKeyboard { disp, xi_opcode }
    }

    pub fn keycode(&mut self) -> i32 {
        loop {
            unsafe {
                let mut event = ::std::mem::MaybeUninit::<XEvent>::zeroed().assume_init();

                XNextEvent(self.disp.disp, &mut event);
                let mut cookie = event.generic_event_cookie;

                if XGetEventData(self.disp.disp, &mut cookie) == true as i32
                    && cookie.type_ == GenericEvent
                    && cookie.extension == self.xi_opcode && cookie.evtype == XI_RawKeyPress
                {
                    let ev = cookie.data as *mut XIRawEvent;
                    return (*ev).detail;
                }
            }
        }
    }
    pub fn keysym(&mut self) -> Option<u64> {
        let k = self.keycode();
        self.keycode2keysym(k)
    }
    pub fn keyname(&mut self) -> Option<&'static str> {
        self.keysym().map(|x| self.keysym2keyname(x))
    }

    pub fn keycode2keysym(&mut self, code: i32) -> Option<u64> {
        let s = unsafe { XkbKeycodeToKeysym(self.disp.disp, code as u8, 0, 0) };
        if s as u64 == NoSymbol as u64 {
            return None;
        }
        Some(s)
    }

    pub fn keysym2keyname(&self, s: u64) -> &'static str {
        let c = unsafe { ::std::ffi::CStr::from_ptr(XKeysymToString(s)) };
        c.to_str().expect("Error in decoding str")
    }
}
