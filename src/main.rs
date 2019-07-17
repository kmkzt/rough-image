extern crate image;

use image::GenericImageView;
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
        writeln!(std::io::stderr(), "Usage rough-image image/path.jpg ...").unwrap();
        std::process::exit(1);
    }

    for path in image_path {
        let img = image::open(path).unwrap();
        // The dimensions method returns the images width and height.
        println!("dimensions {:?}", img.dimensions());

        // The color method returns the image's `ColorType`.
        println!("{:?}", img.color());

        // Write the contents of this image to the Writer in PNG format.
        img.save("test.png").unwrap();
    }
}