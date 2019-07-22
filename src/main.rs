extern crate image;
extern crate gif;

use image::{GenericImageView, DynamicImage};
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::io::Write;
use std::env::args;
use std::fs::File;
use std::borrow::Cow;
use std::vec::Vec;

fn main() {
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let mut image_path: Vec<String> = Vec::new();

    for arg in args().skip(1) {
        image_path.push(arg);
    }

    if image_path.len() == 0 {
        writeln!(std::io::stderr(), "Usage: rough-image image/path.jpg ...").unwrap();
        std::process::exit(1);
    }

    for path in image_path {
        let img = image::open(&path).unwrap();
        println!("path {:?}", &path);
        display_img_info(&img);
        // display_pixels(&img);
        // Write the contents of this image to the Writer in PNG format.
        img.save("test.png").unwrap();
        gen_red_image(&img);
        gen_rgb_gif(&img);
        gen_cube_image(&img, 10);
    }
}

fn display_img_info(img: &DynamicImage ) {
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());
}

// fn display_pixels(img: &DynamicImage ) {
//     let img_size = img.dimensions();
// 
//     for x in 0..img_size.0 {
//         for y in 0..img_size.1 {
//             println!("{:?}", img.get_pixel(x, y) );
//         }
//     }
// }

fn gen_red_image(img: &DynamicImage) {
    let (imgx, imgy) = img.dimensions();
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let data = img.get_pixel(x, y);
            *pixel = image::Rgb([data[0], 0, 0]);
        }
    }
    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("clone.png").unwrap();
}

fn gen_cube_image(img: &DynamicImage, boxsize: u32) {
    let (imgx, imgy) = img.dimensions();
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let loopx = imgx / boxsize;
    let loopy = imgy / boxsize;
    // A redundant loop to demonstrate reading image data
    for x in 0..loopx {
        for y in 0..loopy {
            let mut red_total: u32 = 0;
            let mut green_total: u32 = 0;
            let mut blue_total: u32 = 0;
            
            for bx in 0..boxsize {
                for by in 0..boxsize {
                    let data = img.get_pixel(x + bx, y + by);
                    red_total += data[0] as u32;
                    green_total += data[1] as u32;
                    blue_total += data[2] as u32;
                }
            }
            let avg_pix = image::Rgb([(red_total / boxsize) as u8, (green_total / boxsize) as u8, (blue_total / boxsize) as u8 ]);
            for bx in 0..boxsize {
                for by in 0..boxsize {
                    let pixel = imgbuf.get_pixel_mut(x + bx, y + by);
                     *pixel = image::Rgb([avg_pix[0], avg_pix[1], avg_pix[2]]);
                }
            }
        }
    }
    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("cube.png").unwrap();
}

fn gen_rgb_gif(img: &DynamicImage) {
    let (imgx, imgy) = img.dimensions();
    
    let color_map = &[
        0xFF, 0, 0,
        0, 0xFF, 0,
        0, 0, 0xFF,
        0, 0, 0
    ];
    let width = imgx as u16;
    let height = imgy as u16;

    println!("{}, {}", width, height);
    let mut gif_data = Vec::new();
    let mut pixels = Vec::new();
    for y in 0..imgy {
        for x in 0..imgx {
            let pixel = img.get_pixel(x, y);
            if pixel[0] > pixel[1] && pixel[0] > pixel[2] {
                pixels.push(0);
            } else if pixel[1] > pixel[0] && pixel[1] > pixel[2] {
                pixels.push(1);
            } else if pixel[2] > pixel[0] && pixel[2] > pixel[1] {
                pixels.push(2);
            } else {
                pixels.push(3);
            }
            // print!("{},{},{} -> ", pixel[0], pixel[1], pixel[2]);
            // println!("{}", pixels[(x * y) as usize]);
        }
    }
    assert_eq!(pixels.len(), (imgy * imgx) as usize);
    gif_data.push(pixels);
    let mut image = File::create("rgb.gif").unwrap();;
    let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    for state in &gif_data {
        let mut frame = Frame::default();
        frame.width = width;
        frame.height = height;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
}