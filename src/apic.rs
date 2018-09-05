// use x86_64::registers::model_specific::Msr;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::ExceptionStackFrame;
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

#[allow(dead_code)]
const IA32_APIC_BASE_MSR: u32 = 0x1B;

const PORT_PIC_MASTER_COMMAND: u16 = 0x20;
const PORT_PIC_SLAVE_COMMAND: u16 = 0xA0;

const COMMAND_EOI: u8 = 0x20;

static MASTER: Mutex<Pic> = Mutex::new(Pic::new(PORT_PIC_MASTER_COMMAND));
static SLAVE: Mutex<Pic> = Mutex::new(Pic::new(PORT_PIC_SLAVE_COMMAND));

pub static TIME: AtomicUsize = AtomicUsize::new(0);

bitflags! {
    pub struct Icw1: u8 {
        const NONE      = 0b0000_0000;
        const ICW4      = 0b0000_0001;
        const SINGLE    = 0b0000_0010;
        const INTERVAL4 = 0b0000_0100;
        const LEVEL     = 0b0000_1000;
        const INIT      = 0b0001_0000;
    }
}

pub struct Pic {
    cmd: Port<u8>,
    data: Port<u8>
}

impl Pic {
    pub const fn new(cmd_port: u16) -> Pic {
        Pic {
            cmd: Port::new(cmd_port),
            data: Port::new(cmd_port + 1)
        }
    }

    pub fn send_eoi(&mut self) {
        unsafe {
            self.cmd.write(COMMAND_EOI);
        }
    }

    pub fn mask_set(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.read_mask();
        mask |= 1 << irq;
        self.write_mask(mask);
    }

    pub fn mask_clear(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.read_mask();
        mask &= !(1 << irq);
        self.write_mask(mask);
    }

    pub fn read_mask(&mut self) -> u8 {
        unsafe {
            self.data.read()
        }
    }

    pub fn write_mask(&mut self, mask: u8) {
        unsafe {
            self.data.write(mask);
        }
    }

    pub fn icw1(&mut self, flags: Icw1) {
        unsafe {
            self.cmd.write((flags | Icw1::INIT).bits());
            iowait();
        }
    }

    pub fn icw2(&mut self, offset: u8) {
        unsafe {
            self.data.write(offset);
            iowait();
        }
    }

    pub fn icw3(&mut self, value: u8) {
        unsafe {
            self.data.write(value);
            iowait();
        }
    }

    pub fn icw4(&mut self, value: u8) {
        unsafe {
            self.data.write(value);
            iowait();
        }
    }
}

pub fn ack(irq: u8) {
    assert!(irq < 16);

    if irq < 8 {
        MASTER.lock().mask_clear(irq);
    } else {
        SLAVE.lock().mask_clear(irq - 8);
    }
}

// fn cpu_get_apic_base() -> u64 {
//     let msr = Msr::new(IA32_APIC_BASE_MSR);
//     msr.read();
// }

pub fn init() {
    let mut master = MASTER.lock();
    let mut slave = SLAVE.lock();

    let mask_master = master.read_mask();
    let mask_slave = slave.read_mask();

    master.icw1(Icw1::ICW4);
    slave.icw1(Icw1::ICW4);

    master.icw2(0x20);
    slave.icw2(0x28);

    master.icw3(4);
    slave.icw3(2);

    master.icw4(1);
    slave.icw4(1);

    master.write_mask(mask_master);
    slave.write_mask(mask_slave);

    master.send_eoi();
    slave.send_eoi();

    // let mut port: Port<u8> = Port::new(0x60);
    // let mut portcmd: Port<u8> = Port::new(0x64);
    // unsafe {
    //     port.write(0xF0);
    //     port.write(0x42);
    //     port.write(0xF0);
    //     port.write(0);
    // }
}

fn iowait() {
    unsafe {
        Port::new(0x80).write(0u8);
        ()
    }
}

fn handle_irq(irq: u8) {
    assert!(irq < 16);

    if irq < 8 {
        let mut master = MASTER.lock();
        master.mask_set(irq);
        master.send_eoi();
    } else {
        let mut master = MASTER.lock();
        let mut slave = SLAVE.lock();
        slave.mask_set(irq - 8);
        master.send_eoi();
        slave.send_eoi();
    }

    println!("PIC IRQ {}", irq);
}

macro_rules! interrupt {
    ($name:ident, $body:block) => (
        pub extern "x86-interrupt" fn $name(_stack_frame: &mut ExceptionStackFrame) $body
    )
}

interrupt!(pit, {
    TIME.fetch_add(::pit::NANOSEC_PER_TICK as usize, Ordering::SeqCst);
    MASTER.lock().send_eoi();
});

interrupt!(keyboard, {
    handle_irq(1);

    let port: Port<u8> = Port::new(0x60);

    unsafe {
        let mut c: u8;
        loop {
            c = port.read();
            if c != 0 {
                break;
            } else {
                println!("Error: {:x}", c);
            }
        }

        println!("Key: {:x}", c);
    }

    ack(1);
});

interrupt!(cascade, {
});

interrupt!(com2, {
    handle_irq(3);
});

interrupt!(com1, {
    handle_irq(4);
});

interrupt!(lpt2, {
    handle_irq(5);
});

interrupt!(floppy, {
    handle_irq(6);
});

interrupt!(lpt1, {
    handle_irq(7);
});

interrupt!(rtc, {
    handle_irq(8);
});

interrupt!(pci1, {
    handle_irq(9);
});

interrupt!(pci2, {
    handle_irq(10);
});

interrupt!(pci3, {
    handle_irq(11);
});

interrupt!(mouse, {
    handle_irq(12);
});

interrupt!(fpu, {
    handle_irq(13);
});

interrupt!(ata1, {
    handle_irq(14);
});

interrupt!(ata2, {
    handle_irq(15);
});
