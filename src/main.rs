#![no_main]
#![no_std]

{% if starter_type == "Hello world" %}
    
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::board::Board;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _board = Board::take().unwrap();
    let mut counter = 0u64;
    loop {
        rprintln!("{}", counter);
        counter += 1;
    }
}

{% endif %}


{% if starter_type == "Blinky" %}
    
// https://github.com/pdx-cs-rust-embedded/blinky-rs/
use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{
    board::Board,
    hal::{
        timer::Timer,
    },
};
use rtt_target::{rtt_init_print, rprintln};                                   
use panic_rtt_target as _;                                                    


enum State {
    LedOn,
    LedOff,
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();

    let mut state = State::LedOff;

    loop {
        state = match state {
            State::LedOff => {
                board.display_pins.row1.set_high().unwrap();
                rprintln!("high");
                State::LedOn
            }
            State::LedOn => {
                board.display_pins.row1.set_low().unwrap();
                rprintln!("low");
                State::LedOff
            }
        };
        timer.delay_ms(500);
    }
}

{% endif %}
