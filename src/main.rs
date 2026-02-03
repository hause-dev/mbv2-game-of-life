#![no_main]
#![no_std]


use cortex_m_rt::entry;
use embedded_hal::{digital::InputPin};
use microbit::{
    board::{Board},
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
    },
};

use rtt_target::{rtt_init_print, rprintln};                                   
use panic_rtt_target as _;                                                    

mod life;
use life::*;

const FRAME_LENGTH_MS: u32 = 100;
const FRAME_RESET_BUFFER: u32 = 5; // number of frames to wait for button/world reset

fn randomize_board(fb: &mut [[u8; 5]; 5], rng: &mut HwRng) {
    #[allow(clippy::needless_range_loop)]
    for row in 0..5 {
        for col in 0..5 {
            let buf = rng.random_u8();
            fb[row][col] = if buf < 128 {1} else {0};
        }
    }
}

fn complement_board(fb: &mut [[u8; 5]; 5]) {
    #[allow(clippy::needless_range_loop)]
    for row in 0..5 {
        for col in 0..5 {
            fb[row][col] = if fb[row][col] == 0 {1} else {0};
        }
    }
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    // board variables
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut rng = HwRng::new(board.RNG);

    // button variables
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    // Reset buffers--these are used to track elapsed frames after
    // game completion state or press of button B.
    // When the variable is equal to FRAME_RESET_BUFFER, that indicates
    // we can unfreeze button B or refresh the world state.
    // this should probably be done with a timer, but this works anyways...
    let mut button_b_frame_buffer = FRAME_RESET_BUFFER;
    let mut world_reset_frame_buffer = FRAME_RESET_BUFFER;

    let mut world = [[0u8; 5]; 5];

    randomize_board(&mut world, &mut rng);

    loop {
        rprintln!("Starting new frame");

        // updating frame buffers here
        if button_b_frame_buffer < FRAME_RESET_BUFFER {
            button_b_frame_buffer += 1;
        }
        if world_reset_frame_buffer < FRAME_RESET_BUFFER {
            world_reset_frame_buffer += 1;
            if world_reset_frame_buffer == FRAME_RESET_BUFFER {
                randomize_board(&mut world, &mut rng);
            }
        }

        if button_a.is_low().unwrap() {
            randomize_board(&mut world, &mut rng);
        } else if button_b.is_low().unwrap() && button_b_frame_buffer == 5 {
            if button_b_frame_buffer == 5 {
                complement_board(&mut world);
                button_b_frame_buffer = 0;
            }
        } else if done(&world) && world_reset_frame_buffer == 5 {
            world_reset_frame_buffer = 0;
        } else if world_reset_frame_buffer == 5 {
            life(&mut world);
        } 

        display.show(&mut timer, world, FRAME_LENGTH_MS);
    }
}


