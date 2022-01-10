use std::mem::transmute;
use std::process::{exit, abort};
use std::ffi::CStr;
use libc::{dlsym, dlerror, c_void, RTLD_NEXT, c_char, c_int, sockaddr, c_uint};

// This code derived from proxychains-ng
unsafe fn load_symbol<T>(sym_name: &str, proxy_function: *mut c_void) -> T {
    let sym_bytes: Vec<_> = sym_name.as_bytes().iter().map(|x| *x as c_char).collect();
    let func_ptr: *mut c_void = dlsym(RTLD_NEXT, sym_bytes.as_slice().as_ptr());

    if func_ptr.is_null() {
        let error = dlerror();
        let error_str = CStr::from_ptr(error);
        eprintln!("Cannot load symbol '{}': {:?}", sym_name, error_str);
        exit(1);
    }

    if func_ptr == proxy_function {
        abort();
    }
    unimplemented!()
}


fn connect_hook(sock: c_int, addr: *const sockaddr, len: c_uint) -> c_int {
    unimplemented!()
}

fn getaddrinfo_hook()

fn main() {
    println!("Hello, world!");
}
