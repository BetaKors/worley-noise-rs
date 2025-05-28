use image::{GrayImage, Luma};
use rand::prelude::*;
use std::path::Path;
use text_io::read;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn rand(
        rand: &mut impl Rng,
        x_range: impl rand::distributions::uniform::SampleRange<i32>,
        y_range: impl rand::distributions::uniform::SampleRange<i32>,
    ) -> Self {
        Self::new(rand.gen_range(x_range), rand.gen_range(y_range))
    }

    fn squared_dist(&self, x: i32, y: i32) -> u32 {
        let a = self.x - x;
        let b = self.y - y;
        (a * a + b * b) as u32
    }
}

fn main() {
    let width = input("Width: ");
    let height = input("Height: ");
    let number_of_points = input("Number of points: ");

    let points = generate_points(width, height, number_of_points);

    let instant = std::time::Instant::now();

    let img = generate_image(width, height, &points);

    let elapsed = instant.elapsed().as_secs_f32();

    let path = format!("img/{}.png", generate_random_characters(10));

    if let Some(dirs) = Path::new(&path).parent() {
        std::fs::create_dir_all(dirs).expect("Couldn't create directories.");
    }

    img.save(&path)
        .expect(&format!("Unable to save image at path {}", &path));

    println!("Done! (image generation took {elapsed:.3}s)");
}

const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

fn generate_random_characters(length: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| ASCII_LOWERCASE.chars().choose(&mut rng).unwrap())
        .collect()
}

// one wouldn't normally use squared distance + that arbritary division by 28 but i think it looks better so i did it here
fn generate_image(width: u32, height: u32, points: &Vec<Point>) -> GrayImage {
    GrayImage::from_fn(width, height, |x, y| {
        let closest = points
            .iter()
            .map(|p| p.squared_dist(x as i32, y as i32))
            .min()
            .unwrap();

        Luma([255 - (closest / 28).clamp(0, 255) as u8])
    })
}

fn generate_points(width: u32, height: u32, number_of_points: u32) -> Vec<Point> {
    let mut rng = rand::thread_rng();

    (0..number_of_points)
        .map(|_| Point::rand(&mut rng, 0..width as i32, 0..height as i32))
        .collect()
}

fn input(msg: &str) -> u32 {
    print!("{msg}");
    read!()
}
