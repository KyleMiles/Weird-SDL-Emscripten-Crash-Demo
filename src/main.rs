mod emscripten;
use crate::emscripten::emscripten::set_main_loop_callback;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::rwops::RWops;
use sdl2::ttf;
use sdl2::ttf::Sdl2TtfContext;

use std::time::{Duration, Instant};

static FONT_DATA: &'static [u8] = include_bytes!("cruft.ttf");
static mut TTF_CONTEXT: Option<Sdl2TtfContext> = None;

pub fn main() {
    // Initialize graphics
    let (window_width, window_height) = emscripten::emscripten::get_canvas_size();

    let sdl_context = sdl2::init().unwrap();
    let window = sdl_context
        .video()
        .unwrap()
        .window(
            "Weird Crash Demo",
            window_width as u32,
            window_height as u32,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    // Initialize fonts
    let texture_creator = canvas.texture_creator();
    unsafe { TTF_CONTEXT = Some(ttf::init().expect("Couldn't initialize FontHandler")) };
    let font = unsafe {
        TTF_CONTEXT
            .as_ref()
            .unwrap()
            .load_font_from_rwops(
                RWops::from_bytes(FONT_DATA).expect("Couldn't load font data"),
                60,
            )
            .unwrap()
    };

    // Game loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut square_count = 4;
    let mut show_text = false;

    let main_loop = move || {
        let t1 = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if square_count > 0 {
                        square_count -= 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    square_count += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    show_text = !show_text;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Render boxes
        canvas.set_draw_color(Color::BLACK);
        let div: i32 = window_width as i32 / (square_count + 1);
        canvas
            .fill_rects(
                &(1..square_count + 1)
                    .map(|i| Rect::new((div * i) - 12, (window_height as i32 / 2) - 12, 24, 24))
                    .collect::<Vec<Rect>>(),
            )
            .unwrap();

        // Render text

        if show_text {
            let text_texture = texture_creator
                .create_texture_from_surface(
                    font.render("Press space to toggle text rendering")
                        .blended(Color::RED)
                        .unwrap(),
                )
                .unwrap();
            let TextureQuery { width, height, .. } = text_texture.query();
            canvas
                .copy(
                    &text_texture,
                    None,
                    Rect::new(
                        (window_width as i32 / 2) - (width as i32 / 2),
                        window_height as i32 / 4,
                        width,
                        height,
                    ),
                )
                .expect("Couldn't render text");
        }

        canvas.present();
        let sleep_time = (1_000_000_000u32 / 61u32) - (Instant::now() - t1).as_nanos() as u32;
        ::std::thread::sleep(Duration::new(0, sleep_time as u32));

        println!(
            "FPS: {}",
            1_000_000_000u128 / (Instant::now() - t1).as_nanos()
        );
    };

    set_main_loop_callback(main_loop);
}
