#![no_main]
#![no_std]

use rtt_target::{rtt_init_print, rprintln};                                   
use panic_rtt_target as _;                                                    

use cortex_m_rt::entry;
use microbit::board::Board;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    loop {
    }
}
