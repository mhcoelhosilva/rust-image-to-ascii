extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Write;

use image::{GenericImageView, Pixel, imageops::FilterType};

fn main() {
    let mut file : String;
    let mut scale : u32;
    if env::args().count() == 3 {
        file = env::args().nth(1).unwrap();
        let scaleResult = env::args().nth(2).unwrap().trim().parse();
        scale = match scaleResult {
            Ok(s) => s,
            Err(error) => panic!("Scale should be integer: {:?}", error),
        };
    } else {
        panic!("Please enter a file and scale (e.g. cargo run ./test/cube.png 10")
    };

    let im = image::open(Path::new(&file)).unwrap();
    let buf = image::imageops::resize(&im, im.width() / scale, 
        im.width() / scale, FilterType::Gaussian);

    let imgx = buf.width() as usize;
    let imgy = buf.height() as usize;

    let mut grid = vec![vec!['X'; imgy]; imgx];

    for x in 0..imgx {
        for y in 0..imgy {
            let pixel = buf.get_pixel(x as u32, y as u32);
            let brightness = pixel.to_luma()[0];
            if brightness == 0 {
                grid[y][x] = '#';
            } else if brightness >= 1 && brightness < 33 {
                grid[y][x] = 'X';
            } else if brightness >= 33 && brightness < 67 {
                grid[y][x] = '%';
            } else if brightness >= 67 && brightness < 100 {
                grid[y][x] = '&';
            } else if brightness >= 100 && brightness < 133 {
                grid[y][x] = '*';
            } else if brightness >= 133 && brightness < 166 {
                grid[y][x] = '+';
            } else if brightness >= 166 && brightness < 200 {
                grid[y][x] = '/';
            } else if brightness >= 200 && brightness < 233 {
                grid[y][x] = '(';
            } else if brightness >= 233 && brightness < 250 {
                grid[y][x] = '\'';
            } else {
                grid[y][x] = ' ';
            }
        }
    }

    let mut text : String = "".to_string();
    for x in 0..imgx {
        let s: String = grid[x].iter().collect();
        text.push_str(&s);
        text.push_str("\n");
    }

    let mut file = File::create("test/output.txt").unwrap();
    file.write_all(text.as_bytes());
}
