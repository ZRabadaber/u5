#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32u575;

#[entry]
fn main() -> ! {
    let mut cnt = 0;
    loop {
        cnt = cnt + 1;
    }
}
