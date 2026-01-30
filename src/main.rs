#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::InputPin, delay::DelayNs};
use microbit::{
    board::Board, display::blocking::Display, hal::timer::Timer
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

    let mut i: usize  = 0;
    for row in &mut random_game_board {
        for cell in row {
            *cell = buf[i]%2;
            i+=1;
        }
    }

    random_game_board
}

fn complement_board(game_board: [[u8; 5]; 5]) -> [[u8; 5]; 5]{
    let mut complemented_board: [[u8; 5]; 5] = [[0; 5]; 5];

    for i in 0..5 {
        for j in 0..5 {
            match game_board[i][j] {
                0 => complemented_board[i][j] = 1,
                1 => complemented_board[i][j] = 0,
                _ => complemented_board[i][j] = 0
            }
        }
    }

    complemented_board
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let board: Board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
    let mut rng: Rng = Rng::new(board.RNG);
    
    // The program starts with a random board.
    let mut game_board: [[u8; 5]; 5] = generate_random_board(&mut rng);
    let mut display: Display = Display::new(board.display_pins);

    let mut b_button_ignored: bool = false;
    let mut ignore_b_counter: u32 = 0;

    loop {
        // The program runs the game at 10 frames per second (updates once per 100ms).
        display.show(&mut timer, game_board, 100);

        // Otherwise, if the program reaches a state where all cells on the board are off, 
        if done(&game_board) {
            rprintln!("life is over");
            // the program waits 5 frames (0.5s).
            timer.delay_ms(500);
            ignore_b_counter = 0;
            rprintln!("life starts over again, ignore b counter is reset.");
            // If it has not received a button press, it then starts with a new random board.
            game_board = generate_random_board(&mut rng);
            continue;
        }

        // The B button is then ignored for 5 frames (0.5s).
        if ignore_b_counter > 0 {
            rprintln!("decrementing ignore b counter {}", ignore_b_counter);
            ignore_b_counter -= 1;
        }
        if ignore_b_counter == 0 && b_button_ignored {
            rprintln!("b button is not ignored");
            b_button_ignored = false;
        }

        // While the A button is held, the board is re-randomized every frame.
        let pressed_a: bool = button_a.is_low().unwrap();
        if pressed_a {
            rprintln!("button a pressed, generating new random board");
            game_board = generate_random_board(&mut rng);
            continue;
        }

        // Otherwise, when the B button is not ignored and is pressed, 
        let pressed_b:bool = button_b.is_low().unwrap();
        if pressed_b && !b_button_ignored {
            // the board is “complemented”: every “on” cell is turned “off” and every “off” cell is turned “on”.
            game_board = complement_board(game_board);
            b_button_ignored = true;
            ignore_b_counter = 5;
            rprintln!("button b pressed, complementing board. timer ignore start = {}", ignore_b_counter);
            continue;
        } else if pressed_b && b_button_ignored {
            rprintln!("button b pressed, but ignored.");
        }


        // Otherwise, normal Life steps are taken.
        life(&mut game_board);
        rprintln!("life continues");
    }
}