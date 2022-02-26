use std::os::raw::{c_int, c_void, c_uchar};

#[no_mangle]
pub extern fn reax_init_runtime() {
    store::init_runtime();
}

#[no_mangle]
pub extern fn reax_init_handler(ptr: *const c_void, f: unsafe extern fn(*const c_int, c_int, *const c_uchar, c_int, *const c_void)) {
    store::init_handler(|subs, ser_val| {
        let subs = subs.into_iter().map(|i| i).collect::<Vec<i32>>();
        unsafe { f(subs.as_ptr(), subs.len() as i32, ser_val.as_ptr() as *const c_uchar, ser_val.len() as i32, ptr) };
    });
}

#[no_mangle]
pub extern fn reax_init_store() {
    store::init_store();
}

#[no_mangle]
pub extern fn reax_user() -> c_int {
    store::user()
}

#[no_mangle]
pub extern fn reax_fetch_user() {
    store::fetch_user();
}

#[no_mangle]
pub extern fn reax_unsubscribe(sub: c_int) {
    store::unsubscribe(sub);
}
