use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent}; 
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::render::{TextureCreator, Texture};
use sdl2::video::WindowContext;
use std::time::Duration;
use std::path::Path;

use std::env; 

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel; 

static IMG_PATH: &str = "assets/album_img.png";
const DEFAULT_SIZE: u32 = 400; 

fn main() {

    let args: Vec<String> = env::args().collect();
    
    // debug
    // println!("{:?}", args);

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    
    let size: u32 = if args.len() == 2 { args[1].parse().unwrap() } else { DEFAULT_SIZE }; 
   
    let window = video.window("hehe", size, size)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::WHITE);
    let texture_creator = canvas.texture_creator();

    let mut texture = update_texture(&texture_creator);
    
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(20)).unwrap();
    watcher.watch(IMG_PATH, RecursiveMode::Recursive).unwrap();
    
    // ugly
    let mut image_changed: bool = false;
    let mut should_change: bool = false;
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } | 
                Event::KeyDown {keycode: Some(Keycode::Q), .. } => {
                    break 'main;
                }
                Event::Window {win_event: WindowEvent::Resized(x, y), ..} => {
                    println!("{}, {}", x, y);
                }
                _ => {} 
            }
        }
        match rx.recv_timeout(Duration::from_millis(20)) {
            Ok(_event) => {
                // println!("{:?}", event);
                // ^^ debug 
                image_changed = true;
            },
            Err(e) => 
                if format!("{:?}", e) != "Timeout".to_string() { 
                    println!("ERROR: {:?}", e);
                }
        }
        
        // very crude FSM
        // could and should be replaced
        // please
        if should_change {
            texture = update_texture(&texture_creator);
            canvas.copy(&texture, None, None).unwrap();
            should_change = false;
        }

        if image_changed {
            should_change = true;
            image_changed = false;
        }
        
        ::std::thread::sleep(Duration::from_millis(1000 / 30));
        canvas.present();
    }
}

fn update_texture(tc: &TextureCreator<WindowContext>) -> Texture<'_> {
    tc.load_texture(Path::new(IMG_PATH)).unwrap()
}
