use sdl2::pixels::Color;
use sdl2::event::Event; 
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::time::Duration;
use std::path::Path;

use std::env; 

use notify::{Watcher, RecursiveMode, DebouncedEvent, watcher};
use std::sync::mpsc::channel; 

static IMG_PATH: &str = "assets/album_img.png";
static INFO_PATH: &str = "assets/info.txt"; 
static ASSET_PATH: &str = "assets/"; 
const DEFAULT_SIZE: u32 = 400; 

fn main() {

    let args: Vec<String> = env::args().collect();
    
    // debug
    // println!("{:?}", args);

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    
    let size: u32 = if args.len() == 2 { args[1].parse().unwrap() } else { DEFAULT_SIZE }; 

    let window = video.window("desktify_rs", size, size)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::WHITE);
    let texture_creator = canvas.texture_creator();
    
    update_texture(&mut canvas, &texture_creator);

    canvas.present();

    let (tx, rx) = channel();
    let mut img_watcher = watcher(tx, Duration::from_millis(20)).unwrap();
    img_watcher.watch(ASSET_PATH, RecursiveMode::Recursive).unwrap();
    
    // ugly
    let mut image_changed: bool = false;
    let mut texture_should_change: bool = false;
    let mut info_changed: bool = false; 
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } | 
                Event::KeyDown {keycode: Some(Keycode::Q), .. } => {
                    break 'main;
                }
                _ => {} 
            }
        }
        match rx.recv_timeout(Duration::from_millis(20)) {
            Ok(event) => {
                // println!("{:?}", event);
                // ^^ debug
                match event {
                    DebouncedEvent::Write(p) | DebouncedEvent::Create(p) => {
                        if p.ends_with("info.txt") {
                            info_changed = true; 
                        } else if p.ends_with("album_img.png") {
                            image_changed = true; 
                        }
                    },    
                    DebouncedEvent::NoticeWrite(..) => println!("ignore"),
                    _ => println!("{:?}", event),
                }
            },
            Err(e) => 
                if format!("{:?}", e) != "Timeout".to_string() { 
                    println!("ERROR: {:?}", e);
                }
        }
        
        // very crude FSM
        // could and should be replaced
        // please
        if texture_should_change {
            update_texture(&mut canvas, &texture_creator);
            texture_should_change = false;
        }
        if image_changed {
            texture_should_change = true;
            image_changed = false;
        }
        if info_changed {
            let title = std::fs::read_to_string(INFO_PATH).unwrap(); 
            canvas.window_mut().set_title(&title).unwrap();
            info_changed = false; 
        }
        
        std::thread::sleep(Duration::from_millis(1000 / 30));
        canvas.present();
    }
}

fn update_texture(canvas: &mut Canvas<Window>, tc: &TextureCreator<WindowContext>) -> () {
    match tc.load_texture(Path::new(IMG_PATH)) {
        Ok(t) => canvas.copy(&t, None, None),
        Err(..) => canvas.fill_rect(canvas.clip_rect()) // assuming always the same error
    }; 
}
