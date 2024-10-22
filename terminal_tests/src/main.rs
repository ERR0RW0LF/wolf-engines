extern crate rand; 
use rand::prelude::*;
use std::io::{self, Write};
use crossterm::execute;
use std::io::stdout;


fn main() {
    let mut rng = rand::thread_rng();
    let half_element = "▀";

    let max_size = crossterm::terminal::size().unwrap();

    let (width, height) = (max_size.0, max_size.1*2);
    print!("\x1B[2J");
    // Create a 16x16 image with random RGB values
    let image: Vec<Vec<[u8; 3]>> = (0..height)
        .map(|_| {
            (0..width)
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

    print!("\n\n");
    print!("\x1b[s");

    
    /* 
    for _i in 0..5 {
        print!("\n");
    }*/
    /*println!(
        "\x1b[38;2;{};{};{}m{}\x1b[0m",
        r,
        g,
        b,
        half_element
    );*/
    //println!("\x1b[38;2;{};{};{}mThis is colored text\x1b[0m", r, g, b);

    // Print text with RGB background color
    /*println!(
        "\x1b[48;2;{};{};{}mThis text has a colored background\x1b[0m",
        r, g, b
    );*/

    //println!("r: {}\ng: {}\nb: {}", r, g, b);

    // Set terminal size to 30 rows and 80 columns
    //set_terminal_size(60, 160);

    // Print some text to demonstrate the new terminal size
    /*for i in 0..50 {
        println!("{}",i);
    }*/

    // Print the terminal size
    //println!("{:?}",crossterm::terminal::size().unwrap());

    // Keep the terminal open
    print!("\x1b[H");
    print!("\x1b[?25l");
    print!("\x1b[?47h");
    // input q to quit
    

    // trying out rendering frames
/*
    // 100 frames of random images
    for _ in 0..100 {
        let image = new_image(width, height);
        let image_string = image_to_string(image, half_element);
        let mut buffer = String::new();
        buffer.push_str("\x1b[H"); // Move cursor to the top-left corner without clearing the screen
        buffer.push_str(&image_string);
        print!("{}", buffer);
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100)); // Add a small delay to see the changes
    }
 */
    // 100 frames of a rectangle moving across the screen
    for i in 0..111 {
        let mut image = empty_image(width, height);
        draw_rectangle(&mut image, i, 0, 10, 10, [255, 0, 0]);
        let image_string = image_to_string(image, half_element);
        let mut buffer = String::new();
        buffer.push_str("\x1b[H"); // Move cursor to the top-left corner without clearing the screen
        buffer.push_str(&image_string);
        print!("{}", buffer);
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10)); // Add a small delay to see the changes
    }


    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "" {
            break;
        };
        //clear_screen();
        //print_image(&image);
        /*for h in 0..image_height / 2 {
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
        }*/

        // 100 frames of a rectangle moving across the screen
        for i in 0..111 {
            let mut image = empty_image(width, height);
            draw_rectangle(&mut image, i, 0, 10, 10, [255, 0, 0]);
            let image_string = image_to_string(image, half_element);
            let mut buffer = String::new();
            buffer.push_str("\x1b[H"); // Move cursor to the top-left corner without clearing the screen
            buffer.push_str(&image_string);
            print!("{}", buffer);
            io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10)); // Add a small delay to see the changes
        }

/*         // Create a new 16x16 image with random RGB values
        let image = new_image(16,16);
        let image_string = image_to_string(image, half_element);


        // Clear the screen and move the cursor to the top-left corner
        let mut buffer = String::new();
        buffer.push_str("\x1b[2J\x1b[H");

        // Add the image string to the buffer
        buffer.push_str(&image_string);

        // Print the buffer to the terminal in one go
        print!("{}", buffer);
        io::stdout().flush().unwrap() */
    }
/* 
    while let mut input = String::new(){
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "q" {
            break;
        }
        
    };
*/
    print!("\x1b[J");
    print!("\x1b[?25h");
}

fn image_to_string(image: Vec<Vec<[u8; 3]>>, half_element: &str) -> String {
    let image_height: usize = image.len();
    let mut image_string = String::new();

    for h in 0..image_height / 2 {
        for w in 0..image[h].len() {
            /*print!(
                "\x1b[38;2;{:?};{:?};{:?}m\x1b[48;2;{:?};{:?};{:?}m{}\x1b[0m",
                image[h * 2][w][0],
                image[h * 2][w][1],
                image[h * 2][w][2],
                image[h * 2 + 1][w][0],
                image[h * 2 + 1][w][1],
                image[h * 2 + 1][w][2],
                half_element
            );*/
            image_string.push_str(&format!(
                "\x1b[38;2;{:?};{:?};{:?}m\x1b[48;2;{:?};{:?};{:?}m{}\x1b[0m",
                image[h * 2][w][0],
                image[h * 2][w][1],
                image[h * 2][w][2],
                image[h * 2 + 1][w][0],
                image[h * 2 + 1][w][1],
                image[h * 2 + 1][w][2],
                half_element
            ));
            //print!("\x1b[0m")
        }
        if h < image_height / 2 - 1 {
            image_string.push_str("\n");
            //print!("\n");
        }
    }
    image_string
}

/* 
fn set_terminal_size(rows: u16, cols: u16) {
    // ANSI escape code to set terminal size
    print!("\x1b[8;{};{}t", rows, cols);
    io::stdout().flush().unwrap();
}
*/

fn clear_screen() {
    print!("\x1b[2J");
    print!("\x1b[H");
    io::stdout().flush().unwrap();
}

fn move_cursor(x: u16, y: u16) {
    print!("\x1b[{};{}H", y, x);
    io::stdout().flush().unwrap();
}

fn hide_cursor() {
    print!("\x1b[?25l");
    io::stdout().flush().unwrap();
}

fn show_cursor() {
    print!("\x1b[?25h");
    io::stdout().flush().unwrap();
}

fn print_image(image: &Vec<Vec<[u8; 3]>>) {
    for h in 0..image.len() / 2  {
        for w in 0..image[h].len() {
            print_double_pixel(image[h * 2][w], image[h * 2 + 1][w]);
        }
        print!("\n");
    }
}

fn print_double_pixel(upper_pixel: [u8; 3], lower_pixel: [u8; 3]) {
    print!(
        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀\x1b[0m",
        upper_pixel[0], upper_pixel[1], upper_pixel[2], 
        lower_pixel[0], lower_pixel[1], lower_pixel[2]
    );
    io::stdout().flush().unwrap();
}

fn new_image(width: u16, height: u16) -> Vec<Vec<[u8; 3]>> {
    (0..height)
        .map(|_| {
            (0..width)
                .map(|_| {
                    [
                        rand::random::<u8>(),
                        rand::random::<u8>(),
                        rand::random::<u8>(),
                    ]
                })
                .collect::<Vec<[u8; 3]>>()
        })
        .collect::<Vec<Vec<[u8; 3]>>>()
}

fn empty_image(width: u16, height: u16) -> Vec<Vec<[u8; 3]>> {
    (0..height)
        .map(|_| {
            (0..width)
                .map(|_| {
                    [0, 0, 0]
                })
                .collect::<Vec<[u8; 3]>>()
        })
        .collect::<Vec<Vec<[u8; 3]>>>()
}

fn draw_rectangle(image: &mut Vec<Vec<[u8; 3]>>, x: u16, y: u16, width: u16, height: u16, color: [u8; 3]) {
    for h in y..y + height {
        for w in x..x + width {
            image[h as usize][w as usize] = color;
        }
    }
}