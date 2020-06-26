use image::{ImageBuffer, Rgb, RgbImage};

fn draw_gradient(imgbuf: &mut RgbImage) -> &mut RgbImage {
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let c1 = (0.3 * x as f32) as u8;
        let c2 = (0.3 * y as f32) as u8;
        *pixel = Rgb([c1, 0, c2]);
    }
    imgbuf
}

fn in_thres(x: u32, X: u32, thres: i32) -> bool {
    (x as i32 - X as i32).abs() < thres
}

fn draw_cross(c_x: u32, c_y: u32, imgbuf: &mut RgbImage) -> &mut RgbImage {
    let black = Rgb([0, 0, 0]);
    let thres = 10;

    let length = imgbuf.width() as i32;
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // draw vertical line with thres thickness
        if in_thres(c_y, y, length / 2) && in_thres(c_x, x, thres) {
            *pixel = black;
        }
        if in_thres(c_y, y, thres) && in_thres(c_x, x, length / 2) {
            *pixel = black;
        }
    }
    imgbuf
}

pub fn run() {
    let imgx = 800;
    let imgy = 800;

    // let scalex = 3.0 / imgx as f32;
    // let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = ImageBuffer::new(imgx, imgy);
    draw_gradient(&mut imgbuf);
    let c_x = imgbuf.width() / 2;
    let c_y = imgbuf.height() / 2;
    draw_cross(c_x, c_y, &mut imgbuf);
    for i in 1..10 {
        draw_cross(i, i, &mut imgbuf);
    }
    // // Iterate over the coordinates and pixels of the image
    // // A redundant loop to demonstrate reading image data
    // for x in 0..imgx {
    //     for y in 0..imgy {
    //         let cx = y as f32 * scalex - 1.5;
    //         let cy = x as f32 * scaley - 1.5;

    //         let c = num_complex::Complex::new(-0.4, 0.6);
    //         let mut z = num_complex::Complex::new(cx, cy);

    //         let mut i = 0;
    //         while i < 255 && z.norm() <= 2.0 {
    //             z = z * z + c;
    //             i += 1;
    //         }

    //         let pixel = imgbuf.get_pixel_mut(x, y);
    //         let image::Rgb(data) = *pixel;
    //         *pixel = image::Rgb([data[0], i as u8, data[2]]);
    //     }
    // }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
