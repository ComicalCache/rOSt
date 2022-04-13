use core::arch::asm;

#[allow(dead_code)]
pub fn io_wait() {
    unsafe {
        asm!("out 0x80,al", in("al") 0i8);
    }
}

#[allow(dead_code)]
pub fn asm_inb(port: u16) -> u8 {
    let data: u8;
    unsafe {
        asm!("in al,dx", in("dx") port, out("al") data);
    }
    data
}

#[allow(dead_code)]
pub fn asm_inw(port: u16) -> u16 {
    let data: u16;
    unsafe {
        asm!("in ax,dx", in("dx") port, out("ax") data);
    }
    data
}

#[allow(dead_code)]
pub fn asm_outb(port: u16, data: u8) {
    unsafe {
        asm!("out dx,al", in("dx") port, in("al") data);
    }
}

#[allow(dead_code)]
pub fn asm_outw(port: u16, data: u16) {
    unsafe {
        asm!("out dx,ax", in("dx") port, in("ax") data);
    }
}
