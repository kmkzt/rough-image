extern crate image;

use image::{GenericImageView, DynamicImage};
use std::io::Write;
use std::env::args;

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
        gen_rough_image(&img);
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

fn gen_rough_image(img: &DynamicImage) {
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