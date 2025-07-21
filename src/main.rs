mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use gif::{Frame, Encoder, Repeat};
use std::fs::File;
use std::borrow::Cow;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const SCALE: u32 = 6;
const FRAME_DELAY_MS: u64 = 100;
const GIF_FRAMES: usize = 100; 

fn main() {
    let (mut rl, thread) = raylib::init()
        .size((WIDTH * SCALE) as i32, (HEIGHT * SCALE) as i32)
        .title("Conway's Game of Life - UVG")
        .build();

    rl.set_target_fps(60);

    let mut fb = Framebuffer::new(WIDTH, HEIGHT, SCALE);
    let mut next_gen = Framebuffer::new(WIDTH, HEIGHT, SCALE);


    spawn_block(&mut fb, 5, 5);
    spawn_beehive(&mut fb, 15, 5);
    spawn_loaf(&mut fb, 25, 5);
    spawn_boat(&mut fb, 35, 5);
    spawn_blinker(&mut fb, 55, 5);
    spawn_toad(&mut fb, 65, 8);
    spawn_beacon(&mut fb, 75, 5);
    spawn_butterfly(&mut fb, 92, 8);
    spawn_pulsar(&mut fb, 5, 18);
    spawn_figure_eight(&mut fb, 25, 20);
    spawn_pentadecathlon(&mut fb, 35, 18);
    spawn_glider(&mut fb, 50, 25);
    spawn_lwss(&mut fb, 60, 22);
    spawn_mwss(&mut fb, 75, 20);
    spawn_gosper_glider_gun(&mut fb, 5, 35);
    spawn_r_pentomino(&mut fb, 50, 40);
    spawn_acorn(&mut fb, 60, 38);
    spawn_hwss(&mut fb, 85, 35);
    spawn_galaxy(&mut fb, 5, 55);
    spawn_glider(&mut fb, 25, 60);
    spawn_block(&mut fb, 45, 62);
    spawn_galaxy(&mut fb, 70, 70);
    spawn_beehive(&mut fb, 5, 75);
    spawn_boat(&mut fb, 15, 78);
    spawn_toad(&mut fb, 35, 80);
    spawn_beacon(&mut fb, 50, 75);
    spawn_clock(&mut fb, 65, 77);
    spawn_block(&mut fb, 90, 78);
    spawn_thunderbird(&mut fb, 70, 55);
    spawn_glider(&mut fb, 80, 15);
    spawn_glider(&mut fb, 20, 85);
    spawn_glider(&mut fb, 90, 50);
    spawn_blinker(&mut fb, 5, 95);
    spawn_blinker(&mut fb, 95, 5);
    spawn_blinker(&mut fb, 12, 65);
    spawn_block(&mut fb, 95, 90);
    spawn_block(&mut fb, 2, 90);
    spawn_beehive(&mut fb, 85, 65);
    spawn_boat(&mut fb, 75, 85);
    spawn_loaf(&mut fb, 40, 85);
    spawn_pi_heptomino(&mut fb, 80, 40);
    spawn_lwss(&mut fb, 5, 10); 
    spawn_butterfly(&mut fb, 2, 2);
    spawn_clock(&mut fb, 92, 92);
    spawn_thunderbird(&mut fb, 90, 2);
    spawn_galaxy(&mut fb, 60, 60);

    
    let mut gif_frames = Vec::new();
    let mut frame_count = 0;
    let mut gif_saved = false;
    
    println!("Grabando GIF automáticamente...");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Actualizacion de la generacion
        for y in 0..HEIGHT as i32 {
            for x in 0..WIDTH as i32 {
                let neighbors = count_alive_neighbors(&fb, x, y);
                let is_alive = fb.get_color(x, y) == Color::WHITE;

                let next_state = match (is_alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                if next_state {
                    next_gen.point(x as u32, y as u32, Color::WHITE);
                } else {
                    next_gen.point(x as u32, y as u32, Color::BLACK);
                }
            }
        }

        next_gen.draw_to_raylib(&mut d);


        if frame_count < GIF_FRAMES {
            let frame_data = capture_frame(&next_gen, WIDTH * SCALE, HEIGHT * SCALE);
            gif_frames.push(frame_data);
            frame_count += 1;
            
    
            if frame_count >= GIF_FRAMES && !gif_saved {
                println!("Guardando GIF...");
                save_gif(&gif_frames, WIDTH * SCALE, HEIGHT * SCALE);
                println!("GIF guardado como 'game_of_life.gif'");
                gif_saved = true;
            }
        } else if gif_saved {
        
            d.draw_text("GIF guardado exitosamente!", 10, 10, 20, Color::GREEN);
        }

        std::mem::swap(&mut fb, &mut next_gen);
        thread::sleep(Duration::from_millis(FRAME_DELAY_MS));
    }
}

fn capture_frame(fb: &Framebuffer, width: u32, height: u32) -> Vec<u8> {
    let mut frame_data = vec![0u8; (width * height * 4) as usize];
    
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let cell_x = x / fb.scale;
            let cell_y = y / fb.scale;
            
            if cell_x < fb.width && cell_y < fb.height {
                let color = fb.get_color(cell_x as i32, cell_y as i32);
                frame_data[idx] = color.r;
                frame_data[idx + 1] = color.g;
                frame_data[idx + 2] = color.b;
                frame_data[idx + 3] = 255;
            }
        }
    }
    
    frame_data
}

fn save_gif(frames: &[Vec<u8>], width: u32, height: u32) {
    let mut image = File::create("game_of_life.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width as u16, height as u16, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    
    for frame_data in frames {

        let mut indexed_pixels = vec![0u8; (width * height) as usize];
        for i in 0..(width * height) as usize {
            indexed_pixels[i] = if frame_data[i * 4] > 128 { 1 } else { 0 };
        }
        
        let mut frame = Frame::default();
        frame.width = width as u16;
        frame.height = height as u16;
        frame.buffer = Cow::Borrowed(&indexed_pixels);
        frame.delay = 10; 
        
       
        frame.palette = Some(vec![0, 0, 0, 255, 255, 255]);
        
        encoder.write_frame(&frame).unwrap();
    }
}

fn count_alive_neighbors(fb: &Framebuffer, x: i32, y: i32) -> u32 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = ((x + dx + WIDTH as i32) % WIDTH as i32) as i32;
            let ny = ((y + dy + HEIGHT as i32) % HEIGHT as i32) as i32;
            if fb.get_color(nx, ny) == Color::WHITE {
                count += 1;
            }
        }
    }
    count
}



fn spawn_block(fb: &mut Framebuffer, x: u32, y: u32) {
    for dx in 0..2 {
        for dy in 0..2 {
            fb.point(x + dx, y + dy, Color::WHITE);
        }
    }
}

fn spawn_beehive(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_loaf(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_boat(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_tub(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(1, 0), (0, 1), (2, 1), (1, 2)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}


fn spawn_blinker(fb: &mut Framebuffer, x: u32, y: u32) {
    for dy in 0..3 {
        fb.point(x, y + dy, Color::WHITE);
    }
}

fn spawn_toad(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_beacon(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(0, 0), (1, 0), (0, 1), (2, 2), (3, 2), (3, 3)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_pulsar(fb: &mut Framebuffer, x: u32, y: u32) {
    let offsets = [
        (2, 0), (3, 0), (4, 0),
        (0, 2), (0, 3), (0, 4),
        (5, 2), (5, 3), (5, 4),
        (2, 5), (3, 5), (4, 5),
    ];
    for &(ox, oy) in &offsets {
        for &(dx, dy) in &[(0, 0), (6, 0), (0, 6), (6, 6)] {
            fb.point(x + dx + ox, y + dy + oy, Color::WHITE);
        }
    }
}

fn spawn_pentadecathlon(fb: &mut Framebuffer, x: u32, y: u32) {
    for i in 0..10 {
        fb.point(x + 1, y + i, Color::WHITE);
    }
    fb.point(x, y + 2, Color::WHITE);
    fb.point(x + 2, y + 2, Color::WHITE);
    fb.point(x, y + 7, Color::WHITE);
    fb.point(x + 2, y + 7, Color::WHITE);
}

fn spawn_clock(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 3)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}


fn spawn_glider(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_lwss(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [
        (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1), (4, 1),
        (4, 2),
        (0, 3), (3, 3),
    ];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_mwss(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
        (0, 1), (5, 1),
        (5, 2),
        (0, 3), (4, 3),
    ];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_hwss(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0),
        (0, 1), (6, 1),
        (6, 2),
        (0, 3), (5, 3),
    ];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}



fn spawn_r_pentomino(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_acorn(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}


fn spawn_gosper_glider_gun(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [
       
        (0, 4), (0, 5), (1, 4), (1, 5),
     
        (10, 4), (10, 5), (10, 6), (11, 3), (11, 7), (12, 2), (12, 8),
        (13, 2), (13, 8), (14, 5), (15, 3), (15, 7), (16, 4), (16, 5), (16, 6),
        (17, 5),       (20, 2), (20, 3), (20, 4), (21, 2), (21, 3), (21, 4), (22, 1), (22, 5),
        (24, 0), (24, 1), (24, 5), (24, 6),
        (34, 2), (34, 3), (35, 2), (35, 3),
    ];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}


fn spawn_figure_eight(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [
        (0, 0), (1, 0), (2, 0),
        (0, 1), (1, 1), (2, 1),
        (0, 2), (1, 2), (2, 2),
        (3, 3), (4, 3), (5, 3),
        (3, 4), (4, 4), (5, 4),
        (3, 5), (4, 5), (5, 5),
    ];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_galaxy(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [
        (0, 0), (1, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0),
        (0, 1), (1, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1),
        (0, 3), (1, 3), (7, 3), (8, 3),
        (0, 4), (1, 4), (7, 4), (8, 4),
        (0, 5), (1, 5), (7, 5), (8, 5),
        (0, 6), (1, 6), (7, 6), (8, 6),
        (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (7, 7), (8, 7),
        (0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (7, 8), (8, 8),
    ];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_butterfly(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [
        (0, 0), (1, 0), (2, 0),
        (0, 1), (2, 1),
        (1, 2),
        (1, 3),
        (0, 4), (2, 4),
        (0, 5), (1, 5), (2, 5),
    ];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_pi_heptomino(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(0, 0), (1, 0), (2, 0), (0, 1), (1, 2)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}

fn spawn_thunderbird(fb: &mut Framebuffer, x: u32, y: u32) {
    let cells = [(1, 0), (2, 0), (3, 0), (1, 2), (2, 2), (3, 2)];
    for (dx, dy) in cells {
        fb.point(x + dx, y + dy, Color::WHITE);
    }
}