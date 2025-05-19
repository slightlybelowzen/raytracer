pub mod vec3;
use indicatif::{ProgressBar, ProgressStyle};
// use vec3::Vec3;

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

            let ired = (255.999 * red) as i32;
            let igreen = (255.999 * green as f32) as i32;
            let iblue = (255.999 * blue as f32) as i32;

            println!("{} {} {}", ired, igreen, iblue);
        }
    }
    bar.finish_and_clear();
    eprintln!("finished processing.");
}
