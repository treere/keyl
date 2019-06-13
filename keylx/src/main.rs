use x11::xlib::{XOpenDisplay, XCloseDisplay, XQueryExtension, XDefaultRootWindow, XSync, XEvent, XNextEvent, XGetEventData, GenericEvent, XkbKeycodeToKeysym, NoSymbol, XKeysymToString};
use x11::xinput2::{XIEventMask, XIAllMasterDevices, XI_LASTEVENT, XISetMask, XI_RawKeyPress, XI_RawKeyRelease, XISelectEvents, XIRawEvent};

fn main() {
    let disp = {
        let display = std::ffi::CString::new(":0").expect("Cannot create display name");
        let disp = unsafe { XOpenDisplay(display.as_ptr()) };
        if disp.is_null() {
            panic!("Cannot open display")
        }
        disp
    };

    let xi_opcode = {
        let extension_name = std::ffi::CString::new("XInputExtension").expect("Cannot create extension name");
        let mut xi_opcode = 0;
        let mut query_event = 0;
        let mut query_error = 0;
        if 0 == unsafe { XQueryExtension(disp, extension_name.as_ptr(), &mut xi_opcode, &mut query_event, &mut query_error) } {
            // XXX Test version >=2
            panic!("X Input extension not available")
        }
        xi_opcode
    };

    {
        let root = unsafe { XDefaultRootWindow(disp) };

        let mut m = unsafe { ::std::mem::MaybeUninit::<XIEventMask>::zeroed().assume_init() };
        m.deviceid = XIAllMasterDevices;
        m.mask_len = ((XI_LASTEVENT) >> 3) + 1;
        let mut v = vec![0; m.mask_len as usize];

        XISetMask(&mut v, XI_RawKeyPress);
        XISetMask(&mut v, XI_RawKeyRelease);
        m.mask = v.as_mut_ptr();

        unsafe {
            XISelectEvents(disp, root, &mut m, 1);
            XSync(disp, false as i32)
        };
    }
    loop {
        unsafe {
            let mut event = ::std::mem::MaybeUninit::<XEvent>::zeroed().assume_init();

            XNextEvent(disp, &mut event);
            let mut cookie = event.generic_event_cookie;

            if XGetEventData(disp, &mut cookie) == true as i32
                && cookie.type_ == GenericEvent
                && cookie.extension == xi_opcode && cookie.evtype == XI_RawKeyPress
            {
                let ev = cookie.data as *mut XIRawEvent;
                let s = XkbKeycodeToKeysym(disp, (*ev).detail as u8, 0, 0);
                if s as u64 == NoSymbol as u64 {
                    continue;
                }
                let str = ::std::ffi::CStr::from_ptr(XKeysymToString(s));
                println!("{:?} - {}", str, s);
            }
        }
    }

    unsafe { XCloseDisplay(disp) };
}
