#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use microbit::Board;
use microbit::hal::{prelude::*, delay::Delay, gpio::Level};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut delay = Delay::new(board.SYST);
    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);
    loop {
        speaker.set_high().unwrap();
        delay.delay_us(500u16);
        speaker.set_low().unwrap();
        delay.delay_us(500u16);
    }
}
