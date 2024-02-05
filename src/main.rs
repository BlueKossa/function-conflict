mod function;
mod networking;

use sdl2::mouse::MouseButton;
use sdl2::render::{Canvas, Texture};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::collections::HashSet;
use std::time::Duration;

pub enum PixelState {
    On = 1,
    Off = 0,
}

/// Chunk is a 64x64 pixel square
struct Chunk<'a> {
    data: [u8; 8*64],
    surface: Surface<'a>,
    texture: Option<Texture<'a>>,
}

impl<'a> Chunk<'a> {
    fn new() -> Self {
        let mut surface = Surface::new(64, 64, sdl2::pixels::PixelFormatEnum::RGB24).unwrap();
        surface.fill_rect(None, Color::RGB(255, 255, 255)).unwrap();
        Chunk { 
            data: [0; 8*64],
            surface,
            texture: None,
        }
    }

    fn randomize_data(&mut self) {
        for i in 0..8*64 {
            self.data[i] = rand::random();
        }
    }
    
    fn set_pixel(&mut self, x: u32, y: u32, state: PixelState) {
        let byte_index = 8 * y + x / 8;
        let bit = x % 8;
        let byte = &mut self.data[byte_index as usize];
        *byte = *byte & !(1 << bit) | ((state as u8) << bit);
    }

    fn flip_pixel(&mut self, x: u32, y: u32) {
        let byte_index = 8 * y + x / 8;
        let bit = x % 8;
        let byte = &mut self.data[byte_index as usize];
        *byte = *byte ^ (1 << bit);
    }

    fn get_pixel(&self, x: u32, y: u32) -> u8 {
        let byte_index = 8 * y + x / 8;
        let bit = x % 8;
        (self.data[byte_index as usize] >> bit) & 1
    }

    fn update_texture(&mut self, texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) {
        for y in 0..64 {
            for x in 0..8 {
                let byte = self.data[y*8 + x];
                for bit in 0..8 {
                    if (byte >> bit) & 1 == 1 {
                        self.surface.fill_rect(sdl2::rect::Rect::new(x as i32 * 8 + bit, y as i32, 1, 1), Color::RGB(0, 0, 0)).unwrap();
                    }
                }
            }
        }
        self.texture = Some(self.surface.as_texture(texture_creator).unwrap());

    }

    fn draw(&self, canvas: &mut Canvas<Window>, offset: (i32, i32)) {
        let texture = self.texture.as_ref().unwrap();
        canvas.copy(&texture, None, sdl2::rect::Rect::new(offset.0, offset.1, 64, 64)).unwrap();
    }
}

struct Map<'a> {
    chunks: Vec<Chunk<'a>>,
    chunks_to_update: HashSet<usize>,
    width: u32,
    height: u32,
}

impl<'a> Map<'a> {
    fn new(width: u32, height: u32) -> Self {
        let mut chunks = Vec::new();
        for _ in 0..width*height {
            chunks.push(Chunk::new());
        }
        Map { chunks, width, height, chunks_to_update: HashSet::new() }
    }

    fn is_pixel_oob(&self, x: u32, y: u32) -> bool {
        x >= self.width * 64 || y >= self.height * 64
    }

    fn get_pixel(&self, x: u32, y: u32) -> u8 {
        let chunk_x_pos = x / 64;
        let chunk_y_pos = y / 64;
        let chunk_index = chunk_y_pos * self.width + chunk_x_pos;
        let chunk = &self.chunks[chunk_index as usize];
        chunk.get_pixel(x % 64, y % 64)
    }

    fn set_pixel(&mut self, x: u32, y: u32, state: PixelState) {
        if self.is_pixel_oob(x, y) {
            return;
        }
        let chunk_x_pos = x / 64;
        let chunk_y_pos = y / 64;
        let chunk_index = chunk_y_pos * self.width + chunk_x_pos;
        let chunk = &mut self.chunks.get_mut(chunk_index as usize);
        if let Some(chunk) = chunk {
            chunk.set_pixel(x % 64, y % 64, state);
            self.chunks_to_update.insert(chunk_index as usize);
        }
    }

    fn update(&mut self, texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) {
        for i in &self.chunks_to_update {
            self.chunks[*i].update_texture(texture_creator);
        }
        self.chunks_to_update.clear();
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        for (i, chunk) in self.chunks.iter().enumerate() {
            let x = (i % self.width as usize) * 64;
            let y = (i / self.width as usize) * 64;
            chunk.draw(canvas, (x as i32, y as i32));
        }
    }

    fn circle(&mut self, x: u32, y: u32, radius: u32) {
        if self.is_pixel_oob(x, y) {
            return;
        }
        for i in 0..radius {
            for j in 0..radius {
                if i*i + j*j < radius*radius {
                    self.set_pixel(x + i, y + j, PixelState::On);
                    self.set_pixel(x.saturating_sub(i), y + j, PixelState::On);
                    self.set_pixel(x + i, y.saturating_sub(j), PixelState::On);
                    self.set_pixel(x.saturating_sub(i), y.saturating_sub(j), PixelState::On);
                }
            }
        }
    }
}


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Function Conflict", 1024, 640)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut map = Map::new(16, 10);
    
    for _ in 0..10 {
        //c.randomize_data();
        let x = rand::random::<u32>() % 1024;
        let y = rand::random::<u32>() % 640;
        let radius = rand::random::<u32>() % 100 + 10;
        map.circle(x, y, radius);
        
    }
    for c in &mut map.chunks {
        c.update_texture(&texture_creator);
    }
    
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(sdl2::rect::Rect::new(0, 0, 1024, 640)).unwrap();
        map.draw(&mut canvas);
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return;
                }
                _ => {}
            }
        }
        let mouse = event_pump.mouse_state();
        if mouse.is_mouse_button_pressed(MouseButton::Left) {
            let x = mouse.x();
            let y = mouse.y();
            map.circle(x as u32, y as u32, 10);
        }
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10000));
        map.update(&texture_creator);
    }
}   