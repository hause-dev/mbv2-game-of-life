#![no_main]
#![no_std]


use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{
    board::{Board, Buttons},
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
    },
};
//use nanorand::{pcg64::Pcg64, Rng, SeedableRng};
use rtt_target::{rtt_init_print, rprintln};                                   
use panic_rtt_target as _;                                                    

mod life;
use life::*;

fn randomize_board(fb: &mut [[u8; 5]; 5], rng: &mut HwRng) {
    for row in 0..5 {
        for col in 0..5 {
            let buf = rng.random_u8();
            fb[row][col] = if buf < 128 {1} else {0};
        }
    }
}

fn complement_board(fb: &mut [[u8; 5]; 5]) {
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

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut rng = HwRng::new(board.RNG);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
    let mut button_b_frame_buffer = 5;

    let mut world = [[0u8; 5]; 5];
    let mut world_reset_frame_buffer = 5;
    randomize_board(&mut world, &mut rng);

    loop {
        rprintln!("Starting new frame");

        // updating frame buffers here
        if button_b_frame_buffer < 5 {
            button_b_frame_buffer += 1;
        }
        if world_reset_frame_buffer < 5 {
            world_reset_frame_buffer += 1;
            if world_reset_frame_buffer == 5 {
                randomize_board(&mut world, &mut rng);
            }
        }

        if button_a.is_low().unwrap() {
            rprintln!("Button A is pressed");
            randomize_board(&mut world, &mut rng);
        } else if button_b.is_low().unwrap() && button_b_frame_buffer == 5 {
            if button_b_frame_buffer == 5 {
                rprintln!("Button B is pressed, complementing");
                complement_board(&mut world);
                button_b_frame_buffer = 0;
            } else {
                rprintln!("Button B is pressed, but frozen");
            }
        } else {
            if done(&world) && world_reset_frame_buffer == 5 {
                world_reset_frame_buffer = 0;
            } else if world_reset_frame_buffer == 5 {
                rprintln!("Making game step");
                life(&mut world);
            } else {
                rprintln!("Waiting for button press or world reset");
            }
        }

        display.show(&mut timer, world, 100);
    }
}


