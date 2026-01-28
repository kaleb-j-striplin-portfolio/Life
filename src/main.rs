#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::{OutputPin,InputPin}, delay::DelayNs};
use microbit::{
    board::Board,
    hal::{
        timer::Timer,
    },
    display::blocking::Display
};

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod life;
use life::*;

enum State {
    LedOn,
    LedOff,
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut button = board.buttons.button_a;
    let game_board_heart = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];
    let empty_board_heart = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
    let mut display = Display::new(board.display_pins);


    // board.display_pins.col1.set_low().unwrap();

    let mut state = State::LedOff;

    loop {
        let pressed = button.is_low().unwrap();
        state = match (pressed, state) {
            (true, State::LedOn) => {
                display.show(&mut timer, game_board_heart, 10);
                rprintln!("heart");
                State::LedOff
            }
            _ => {
                display.show(&mut timer, empty_board_heart, 10);
                rprintln!("none");
                State::LedOn
            }
        };
        timer.delay_ms(100);
    }
}