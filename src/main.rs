#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::InputPin, delay::DelayNs};
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

fn generate_random_board(rng: &mut Rng) -> [[u8; 5]; 5] {
    let mut random_game_board: [[u8; 5]; 5] = [[0u8; 5]; 5];
    let mut buf: [u8; 25] = [0; 25];
    rng.random(&mut buf);

    rprintln!("buffer {:?}", buf);
    let mut i: usize  = 0;
    for row in &mut random_game_board {
        for cell in row {
            *cell = buf[i]%2;
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
    let mut button_a = board.buttons.button_a;
    let mut rng: Rng = Rng::new(board.RNG);
    
    // The program starts with a random board.
    let mut game_board: [[u8; 5]; 5] = generate_random_board(&mut rng);
    let mut display: Display = Display::new(board.display_pins);

    loop {
        
        let pressed_a: bool = button_a.is_low().unwrap();
        if pressed_a {
            game_board = generate_random_board(&mut rng);
            display.show(&mut timer, game_board, 100);
            rprintln!("new board");
        }

        // Otherwise, normal Life steps are taken.
        life(&mut game_board);

        // The program runs the game at 10 frames per second (updates once per 100ms).
        display.show(&mut timer, game_board, 100);
        
        // Otherwise, if the program reaches a state where all cells on the board are off, 
        if done(& game_board) {
            // the program waits 5 frames (0.5s). 
            // If it has not received a button press, it then starts with a new random board.
            timer.delay_ms(500);
            game_board = generate_random_board(&mut rng);
        }
    }
}