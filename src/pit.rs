use x86_64::instructions::port::Port;

static SELECT_CHAN0: u8 = 0;
static LOHI: u8 = 0x30;

const PIT_TICK_RATE_HZ: u64 = 1_193_182;
const HZ: u64 = 100;
const DIVISOR: u16 = (PIT_TICK_RATE_HZ as f64 / HZ as f64) as u16;
pub const NANOSEC_PER_TICK: u64 = (1_000_000_000f64 / (PIT_TICK_RATE_HZ as f64 / DIVISOR as f64)) as u64;

pub fn init() {
  let mut cmd = Port::new(0x43);
  let mut ch0 = Port::new(0x40);

  println!("PIT Data: HZ={}, DIVISOR={}, NANOSEC_PER_TICK={}", HZ, DIVISOR, NANOSEC_PER_TICK);

  unsafe {
      cmd.write(SELECT_CHAN0 | LOHI | 6);
      ch0.write((DIVISOR & 0xFF) as u8);
      ch0.write((DIVISOR >> 8) as u8);
  }
}
