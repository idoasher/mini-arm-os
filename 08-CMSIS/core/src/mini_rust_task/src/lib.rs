#![no_std]
// alloc error handler is unstable feature and we need feature gate to enable it
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(const_fn)]

extern crate alloc;
mod mini_alloc;
mod mini_panic_handler;
mod mini_stream;

use alloc::string::*;
use cortex_m::asm;
use mini_stream::puts;

#[global_allocator]
static A: mini_alloc::MiniAllocator = mini_alloc::MiniAllocator::empty();

#[no_mangle]
pub extern "C" fn rust_task() {
    puts("Task3: BEGIN RUST\n\0");
    let mut s = "Task3: Printed by RUST\n\0".to_string();

    loop {
        puts(&s);
        s.pop();
        s.pop();
        s = s + "!\n\0";
        for _ in 1..1000000 {
            asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
        }
    }
}
