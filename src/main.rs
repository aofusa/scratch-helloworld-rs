#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use sc::syscall;
use core::arch::asm;

fn write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        // syscall!(WRITE, fd, buf as usize, len);
        // x86_64のsyscall呼び出しを直接アセンブラで書いてみる
        let mut _write: usize = 1;
        asm!(
            "syscall",
            inout("rax") _write,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") len,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
}

fn exit(status: usize) -> ! {
    unsafe {
        syscall!(EXIT, status);
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // loop {}
    let msg = "Hello, world!\n";
    let buf = msg.as_ptr();
    let len = msg.len();

    write(1, buf, len);
    exit(0);
}
