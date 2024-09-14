use std::fs::{self, File};
use std::io::{self, Write};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const TOTAL_PIXELS: usize = WIDTH * HEIGHT;
const TOTAL_FRAMES: usize = 5000;
const FRAME_DIR: &str = "frames";
const RING_THICKNESS: f64 = 10.0;  
const GROWTH_RATE: f64 = 5.0;  

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

//  binary map 
fn generate_ring_frame(frame: usize, max_radius: f64) -> Vec<u8> {
    let mut binary_map = vec![0; TOTAL_PIXELS / 8];

    let center_x = WIDTH as f64 / 2.0;
    let center_y = HEIGHT as f64 / 2.0;
    
    // outer and inner radius of the ring
    let outer_radius = (frame as f64 * GROWTH_RATE) % max_radius;
    let inner_radius = (outer_radius - RING_THICKNESS).max(0.0);  

    // Iterate over every pixel
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let distance_from_center = distance(x as f64, y as f64, center_x, center_y);
            if distance_from_center >= inner_radius && distance_from_center <= outer_radius {
                let index = y * WIDTH + x;
                let byte_index = index / 8;
                let bit_position = index % 8;

                binary_map[byte_index] |= 1 << bit_position;  
            }
        }
    }

    binary_map
}

fn main() -> io::Result<()> {
    fs::create_dir_all(FRAME_DIR)?;

    let max_radius = (WIDTH as f64 / 2.0).min(HEIGHT as f64 / 2.0);  

    for frame in 0..TOTAL_FRAMES {
        let binary_map = generate_ring_frame(frame, max_radius);
        let filename = format!("{}/frame_{}.bin", FRAME_DIR, frame);
        let mut file = File::create(&filename)?;
        file.write_all(&binary_map)?;
    }

    println!("{} frames generated.", TOTAL_FRAMES);
    Ok(())
}
