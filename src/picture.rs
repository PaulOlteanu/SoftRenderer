use std::cmp;

use crate::model::Model;

pub fn render_model((resolution_x, resolution_y): (u32, u32), file_path: &str, model: &Model) {
    let mut image_buffer = image::RgbImage::new(resolution_x, resolution_y);

    let verts = model.verts();
    for face in model.faces() {
        let v1 = verts[face[0] as usize];
        let v1 = (
            ((v1.0 + 1.0) * resolution_x as f64 / 2.0) as i64,
            ((v1.1 + 1.0) * resolution_y as f64 / 2.0) as i64,
            );
        let v2 = verts[face[1] as usize];
        let v2 = (
            ((v2.0 + 1.0) * resolution_x as f64 / 2.0) as i64,
            ((v2.1 + 1.0) * resolution_y as f64 / 2.0) as i64,
            );
        let v3 = verts[face[2] as usize];
        let v3 = (
            ((v3.0 + 1.0) * resolution_x as f64 / 2.0) as i64,
            ((v3.1 + 1.0) * resolution_y as f64 / 2.0) as i64,
            );

        triangle(v1, v2, v3, &mut image_buffer, (255, 255, 255));
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

fn triangle(
    v1: (i64, i64),
    v2: (i64, i64),
    v3: (i64, i64),
    image: &mut image::RgbImage,
    color: (u8, u8, u8),
) {
    let x_min = cmp::min(cmp::min(cmp::min(0, v1.0), v2.0), v3.0) as i64;
    let x_max = cmp::max(
        cmp::max(cmp::max(image.width() as i64 - 1, v1.0), v2.0),
        v3.0,
    );
    let y_min = cmp::min(cmp::min(cmp::min(0, v1.1), v2.1), v3.1) as i64;
    let y_max = cmp::max(
        cmp::max(cmp::max(image.height() as i64 - 1, v1.1), v2.1),
        v3.1,
    );

    for x in x_min..x_max {
        for y in y_min..y_max {
            if is_point_in_triangle(v1, v2, v3, (x, y)) {
                image.put_pixel(x as u32, y as u32, image::Rgb([color.0, color.1, color.2]))
            }
        }
    }
}

fn is_point_in_triangle(v1: (i64, i64), v2: (i64, i64), v3: (i64, i64), point: (i64, i64)) -> bool {
    let vec1 = (
        v3.0 as f64 - v1.0 as f64,
        v2.0 as f64 - v1.0 as f64,
        v1.0 as f64 - point.0 as f64,
    );

    let vec2 = (
        v3.1 as f64 - v1.1 as f64,
        v2.1 as f64 - v1.1 as f64,
        v1.1 as f64 - point.1 as f64,
    );

    let u = (
        vec1.1 * vec2.2 - vec1.2 * vec2.1, // X = YZ-ZY
        vec1.2 * vec2.0 - vec1.0 * vec2.2, // Y = ZX-XZ
        vec1.0 * vec2.1 - vec1.1 * vec2.0, // Z = XY-YX
    );

    if u.2.abs() < 1.0 {
        return false;
    }

    let barycentric = (1.0 - (u.0 + u.1) / u.2, u.1 / u.2, u.0 / u.2);

    if barycentric.0 < 0.0 || barycentric.1 < 0.0 || barycentric.2 < 0.0 {
        false
    } else {
        true
    }
}
