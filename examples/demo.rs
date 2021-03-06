extern crate sdl2;
extern crate sdl2_ttf;

use std::env;
use std::path::Path;

use sdl2::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

static SCREEN_WIDTH : i32 = 800;
static SCREEN_HEIGHT : i32 = 600;

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn run(filename: &Path) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    sdl2_ttf::init();

    let window = video_subsys.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    // Load a font
    let font = trying!(sdl2_ttf::Font::from_file(filename, 128));

    // render a surface, and convert it to a texture bound to the renderer
    let surface = trying!(font.render_str_blended("Hello Rust!", sdl2::pixels::Color::RGBA(255, 0, 0, 255)));
    let mut texture = trying!(renderer.create_texture_from_surface(&surface));

    renderer.set_draw_color(sdl2::pixels::Color::RGBA(195, 217, 255, 255));
    renderer.clear();

    let (w, h) = { let q = texture.query(); (q.width, q.height) };

    renderer.copy(&mut texture, None, Some(rect!((SCREEN_WIDTH as u32 - w)/ 2, (SCREEN_HEIGHT as u32 - h)/ 2, w, h)).unwrap().unwrap());

    renderer.present();

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} => break 'mainloop,
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => break 'mainloop,
                _ => {}
            }
        }
    }

    sdl2_ttf::quit();
}

fn main() {
    let args: Vec<_> = env::args().collect();

    println!("linked sdl2_ttf: {}", sdl2_ttf::get_linked_version());

    if args.len() < 2 {
        println!("Usage: ./demo font.[ttf|ttc|fon]")
    } else {
        let path: &Path = Path::new(&args[1]);
        run(path);
    }
}
