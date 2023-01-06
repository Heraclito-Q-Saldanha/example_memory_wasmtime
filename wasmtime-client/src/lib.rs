#[no_mangle]
pub fn hello() -> *mut u8{
    let mut buf = "h3ll0".to_string();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr    
}