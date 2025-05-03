use core::*;
use std::env;
use sdl2::event::Event;
use std::fs::File;
use std::io::Read;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32)*SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32)*SCALE;
const TICK_PER_SECOND: usize = 10;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Error: Missing path to game ROM.\n");
        return;
    }


    // Set up SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut emu = Emu::new();
    let mut rom = File::open(&args[1]).expect("Fail to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer ).unwrap();
    emu.load(&buffer);

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. } => break 'gameloop,
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(k) = key2btn(key) {
                        emu.keypress(k, true);
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(k) = key2btn(key) {
                        emu.keypress(k, false);
                    }
                },
                _ => ()
            }
        }

        for _ in 0..TICK_PER_SECOND {
            emu.tick();
        }

        emu.tick_timer();
        draw_screen(&mut canvas, &emu);
    }
}


fn draw_screen(canvas: &mut Canvas<Window>, emu: &Emu) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_screen();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * SCALE) as i32,(y * SCALE) as i32,SCALE,SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}