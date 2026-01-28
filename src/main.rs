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
use nrf52833_hal::Rng;


use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod life;
use life::*;

enum State {
    LedOn,
    LedOff,
}

fn generate_random_board(rng: &mut Rng) -> [[u8; 5]; 5] {
    let mut random_game_board: [[u8; 5]; 5] = [[0u8; 5]; 5];
    let mut buf: [u8; 25] = [0; 25];
    rng.random(&mut buf);

    rprintln!("buffer {:?}", buf);
    let mut i = 0;
    for y in 0..5 {
        for x in 0..5 {
            random_game_board[y][x] = buf[i]%2;
            i+=1;
        }
    }
    rprintln!("board {:?}", random_game_board);
    random_game_board
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let board: Board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut button = board.buttons.button_a;
    let _game_board_heart: [[u8; 5]; 5] = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];
    let empty_board_heart: [[u8; 5]; 5] = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
    let mut rng: Rng = Rng::new(board.RNG);
    
    // The program starts with a random board.
    let mut random_game_board: [[u8; 5]; 5] = generate_random_board(&mut rng);
    let mut display: Display = Display::new(board.display_pins);
    let mut state: State = State::LedOff;

    loop {
        
        // let pressed = button.is_low().unwrap();
        // state = match (pressed, state) {
        //     (true, State::LedOn) => {
        //         display.show(&mut timer, random_game_board, 10);
        //         rprintln!("heart");
        //         State::LedOff
        //     }
        //     _ => {
        //         display.show(&mut timer, empty_board_heart, 10);
        //         rprintln!("none");
        //         State::LedOn
        //     }
        // };


        // Otherwise, normal Life steps are taken.
        life(&mut random_game_board);
        
        // The program runs the game at 10 frames per second (updates once per 100ms).
        display.show(&mut timer, random_game_board, 100);
        
        // Otherwise, if the program reaches a state where all cells on the board are off, 
        if done(& random_game_board) {
            // the program waits 5 frames (0.5s). 
            // If it has not received a button press, it then starts with a new random board.
            timer.delay_ms(500);
            random_game_board = generate_random_board(&mut rng);
        }

        // timer.delay_ms(10);
    }
}