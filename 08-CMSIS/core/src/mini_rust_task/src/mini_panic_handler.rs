use crate::mini_stream::puts;
use core::panic::PanicInfo;

extern "C" {
    pub fn thread_self_terminal() -> !;
}

#[panic_handler]
fn panic_thread_termination(_info: &PanicInfo) -> ! {
    puts("Panic occurred. Terminating task!\n\0");
    unsafe { thread_self_terminal() }
}
