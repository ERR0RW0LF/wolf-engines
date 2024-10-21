extern crate rand;
use rand::prelude::*;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::thread_rng();
    let half_element = "▀";

    // Generate random RGB values
    let r: u8 = rng.gen_range(0..=255);
    let g: u8 = rng.gen_range(0..=255);
    let b: u8 = rng.gen_range(0..=255);

    // Create a 16x16 image with random RGB values
    let image: Vec<Vec<[u8; 3]>> = (0..16)
        .map(|_| {
            (0..16)
                .map(|_| {
                    [
                        rng.gen_range(0..=255),
                        rng.gen_range(0..=255),
                        rng.gen_range(0..=255),
                    ]
                })
                .collect::<Vec<[u8; 3]>>()
        })
        .collect::<Vec<Vec<[u8; 3]>>>();

    let image_height: usize = image.len();

    println!("{:?}", image_height);

    // Print text with RGB foreground color
    for i in 0..127 {
        print!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            r.saturating_add(i * 2),
            g.saturating_add(i * 2),
            b.saturating_add(i * 2),
            r.saturating_add(i * 2 + 1),
            g.saturating_add(i * 2 + 1),
            b.saturating_add(i * 2 + 1),
            half_element
        );
    }
    print!("\n\n");

    for h in 0..image_height / 2 {
        for w in 0..image[h].len() {
            print!(
                "\x1b[38;2;{:?};{:?};{:?}m\x1b[48;2;{:?};{:?};{:?}m{}\x1b[0m",
                image[h * 2][w][0],
                image[h * 2][w][1],
                image[h * 2][w][2],
                image[h * 2 + 1][w][0],
                image[h * 2 + 1][w][1],
                image[h * 2 + 1][w][2],
                half_element
            );
        }
        print!("\n");
    }
    for _i in 0..5 {
        print!("\n");
    }
    println!(
        "\x1b[38;2;{};{};{}m{}\x1b[0m",
        r,
        g,
        b,
        half_element
    );
    println!("\x1b[38;2;{};{};{}mThis is colored text\x1b[0m", r, g, b);

    // Print text with RGB background color
    println!(
        "\x1b[48;2;{};{};{}mThis text has a colored background\x1b[0m",
        r, g, b
    );

    println!("r: {}\ng: {}\nb: {}", r, g, b);

    // Set terminal size to 30 rows and 80 columns
    set_terminal_size(30, 80);

    // Print some text to demonstrate the new terminal size
    for _ in 0..30 {
        println!("This is a line of text.");
    }

    // Keep the terminal open
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn set_terminal_size(rows: u16, cols: u16) {
    // ANSI escape code to set terminal size
    print!("\x1b[8;{};{}t", rows, cols);
    io::stdout().flush().unwrap();
}