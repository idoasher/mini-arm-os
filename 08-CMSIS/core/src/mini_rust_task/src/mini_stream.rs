extern crate cty;

#[repr(C)]
enum STREAM {
    USART,
}

extern "C" {
    fn stream_write(stream_type: STREAM, data: *const cty::c_char);
}

pub fn puts(out: &str) {
    unsafe {
        stream_write(STREAM::USART, out.as_ptr());
    }
}
