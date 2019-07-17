extern crate image;

use image::GenericImageView;
use image::DynamicImage;
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
        display_pixels(&img);
        // Write the contents of this image to the Writer in PNG format.
        img.save("test.png").unwrap();
    }
}

fn display_img_info(img: &DynamicImage ) {
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());
}

fn display_pixels(img: &DynamicImage ) {
    let img_size = img.dimensions();
    for x in 0..img_size.0 {
        for y in 0..img_size.1 {
            println!("{:?}", img.get_pixel(x, y) );
        }
    }
}