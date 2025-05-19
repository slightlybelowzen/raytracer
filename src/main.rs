pub mod color;
use color::Color;
use indicatif::{ProgressBar, ProgressStyle};

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    let bar = ProgressBar::new_spinner();
    bar.set_style(
        ProgressStyle::with_template("{spinner:.green.dim} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );

    for j in 0..IMAGE_HEIGHT {
        bar.set_message(format!("processed {} out of {}", j, IMAGE_HEIGHT));
        for i in 0..IMAGE_WIDTH {
            let red = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let green = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let blue: f32 = 0.0;

            let color = Color::new(red, green, blue);
            println!("{}", color);
        }
    }
    bar.finish_and_clear();
    eprintln!("finished processing.");
}
