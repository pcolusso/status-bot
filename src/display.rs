use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "./libwaveshare.so")]
extern {
    fn EPD_2in13_displayMessage(message: *const c_char);
    fn EPD_2in13_displayImage(image: [u8]);
}

pub fn renderMessage(message: String) {
    let as_cstr = CString::new(message).unwrap();
    unsafe {
        EPD_2in13_displayMessage(as_cstr.as_ptr());
    }
}