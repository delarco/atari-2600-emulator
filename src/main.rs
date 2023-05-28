mod cpu;
use cpu::CPU;
use sdl2::event::Event;
use sdl2::{rect::Rect, keyboard::Keycode, pixels::Color};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    env_logger::init();

    let cpu = CPU::new();

    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem
        .window("Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let screen_area = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut event_queue = context.event_pump().unwrap();
    let mut running = true;

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => running = false,
                _ => {}
            }
        }

        let clear_color = Color::RGB(255,192,64);
        canvas.set_draw_color(clear_color);
        canvas.fill_rect(screen_area).ok().unwrap();
        canvas.present();
    }
}
