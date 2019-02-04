use std::cmp;

use crate::model::Model;

pub fn render_model((resolution_x, resolution_y): (u32, u32), file_path: &str, model: &Model) {
    let mut image_buffer = image::RgbImage::new(resolution_x, resolution_y);

    let verts = model.verts();
    for face in model.faces() {
        for j in 0..3 {
            let start = verts[face[j] as usize];
            let end = verts[face[(j + 1) % 3] as usize];

            let start = (
                cmp::min(
                    ((start.0 + 1.0) * resolution_x as f64 / 2.0) as i64,
                    resolution_x as i64 - 1,
                ),
                cmp::min(
                    ((start.1 + 1.0) * resolution_y as f64 / 2.0) as i64,
                    resolution_y as i64 - 1,
                ),
            );

            let end = (
                cmp::min(
                    ((end.0 + 1.0) * resolution_x as f64 / 2.0) as i64,
                    resolution_x as i64 - 1,
                ),
                cmp::min(
                    ((end.1 + 1.0) * resolution_y as f64 / 2.0) as i64,
                    resolution_y as i64 - 1,
                ),
            );

            line(start, end, &mut image_buffer, (255, 255, 255));
        }
    }

    image_buffer = image::imageops::flip_vertical(&image_buffer);
    image_buffer.save(format!("{}.png", file_path)).unwrap();
}

fn line(start: (i64, i64), end: (i64, i64), image: &mut image::RgbImage, color: (u8, u8, u8)) {
    let x1 = start.0;
    let y1 = start.1;
    let x2 = end.0;
    let y2 = end.1;
    let steep: bool;

    // If dy > dx then the line is "steep"
    // The algorithm works in the 1st octant
    // If the slope is greater than that, reflect the line over y=x, then flip it when outputting the points
    let ((x1, y1), (x2, y2)) = if (x1 - x2).abs() < (y1 - y2).abs() {
        steep = true;
        ((y1, x1), (y2, x2))
    } else {
        steep = false;
        ((x1, y1), (x2, y2))
    };

    // If the endpoint is after the start point, swap them
    let ((x1, y1), (x2, y2)) = if x1 > x2 {
        ((x2, y2), (x1, y1))
    } else {
        ((x1, y1), (x2, y2))
    };

    let dx = x2 - x1;
    let dy = y2 - y1;

    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut y = y1;

    for x in x1..x2 + 1 {
        if steep {
            // Unswap x and y
            image.put_pixel(y as u32, x as u32, image::Rgb([color.0, color.1, color.2]));
        } else {
            image.put_pixel(x as u32, y as u32, image::Rgb([color.0, color.1, color.2]));
        }

        error += derror;

        if error > dx {
            y += if y2 > y1 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}
