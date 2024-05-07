mod image;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut ppm_image = image::PPMImage::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = ((x + y) / 2) as f64 / ((image_width + image_height) / 2) as f64;

            ppm_image.pixels[y][x] = image::PPMPixel::new(r, g, b);
        }
    }

    let _ = ppm_image.write_to_file("output.ppm");
}
