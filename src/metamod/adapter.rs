use std::ffi::CString;

use super::meta;

pub fn err_log(msg: &str) {
    if let Ok(cmsg) = CString::new(msg) {
        meta::err_log(cmsg.as_c_str());
    }
}

pub fn console_log(msg: &str) {
    if let Ok(cmsg) = CString::new(msg) {
        meta::console_log(cmsg.as_c_str());
    }
}

pub fn alert(msg: &str) {
    if let Ok(cmsg) = CString::new(msg) {
        meta::alert(cmsg.as_c_str());
    }
}

pub fn client_print(msg: &str) {
    
}
