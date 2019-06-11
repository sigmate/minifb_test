extern crate minifb;

use minifb::{Key, WindowOptions, Window};

// Framebuffer dimensions
const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Plasma - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut time: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        for y in 0..HEIGHT
        {

            for x in 0..WIDTH
            {
                // Normalize x and y to -0.5:0.5
                let xf: f32 = (x as f32) / (WIDTH as f32) - 0.5;
                let yf: f32 = ((HEIGHT - y) as f32) / (HEIGHT as f32) - 0.5;

                // Calculate v1, v2, v3: three sin-based functions
                let v1: f32 = f32::sin(xf * 10.0 + time);
                let v2: f32 = f32::sin(10.0 * (xf * f32::sin(time / 2.0) + yf * f32::cos(time / 3.0)) + time);
                let cx: f32 = xf + 0.5 * f32::sin(time / 5.0);
                let cy: f32 = yf + 0.5 * f32::cos(time / 3.0);
                let v3: f32 = f32::sin(f32::sqrt(100.0 * ((cx * cx) + (cy * cy)) + 1.0 + time));

                // Finally, set our buffer with a sum of our three functions, and some sin-based color (grey) mapping applied.
                buffer[(y * WIDTH) + x] = map_trig_float_to_grey(f32::sin((v1+v2+v3) * std::f32::consts::PI * f32::sin(time * 0.05) * 3.0));
            }

        }
        
        window.update_with_buffer(&buffer).unwrap();

        // Update our time variable for animation
        time += 0.02;

    }
}

// Maps a -1:1 value to a grey u32 ARGB value 
fn map_trig_float_to_grey(v: f32) -> u32 {
    let u: u32 = (((v / 2.0) + 0.5) * 255.0) as u32;
    u + (u << 8) + (u << 16)
}