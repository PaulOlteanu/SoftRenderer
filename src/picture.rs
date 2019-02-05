use std::cmp;

use cgmath::{Vector2, Vector3, Matrix2};

use crate::model::Model;

pub fn render_model((resolution_x, resolution_y): (u32, u32), file_path: &str, model: &Model) {
    let mut image_buffer = image::RgbImage::new(resolution_x, resolution_y);

    let verts = model.verts();

    let half_width = resolution_x as f64 / 2.0;
    let half_height = resolution_y as f64 / 2.0;

    let coordinate_transform_matrix = Matrix2::new(half_width, 0.0, 0.0, half_height);
    let increment = Vector2::new(1.0, 1.0);

    for face in model.faces() {
        let v1: Vector2<u32> = (coordinate_transform_matrix * (verts[face[0]].xy() + increment)).cast().unwrap();
        let v2: Vector2<u32> = (coordinate_transform_matrix * (verts[face[1]].xy() + increment)).cast().unwrap();
        let v3: Vector2<u32> = (coordinate_transform_matrix * (verts[face[2]].xy() + increment)).cast().unwrap();

        triangle(v1, v2, v3, &mut image_buffer, image::Rgb([255, 255, 255]));
    }

    image_buffer = image::imageops::flip_vertical(&image_buffer);
    image_buffer.save(format!("{}.png", file_path)).unwrap();
}

fn line(start: (i64, i64), end: (i64, i64), image: &mut image::RgbImage, color: image::Rgb<u8>) {
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
            image.put_pixel(y as u32, x as u32, color);
        } else {
            image.put_pixel(x as u32, y as u32, color);
        }

        error += derror;

        if error > dx {
            y += if y2 > y1 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

fn triangle(
    v1: Vector2<u32>,
    v2: Vector2<u32>,
    v3: Vector2<u32>,
    image: &mut image::RgbImage,
    color: image::Rgb<u8>,
) {
    let x_min = cmp::max(0, cmp::min(cmp::min(v1.x, v2.x), v3.x));
    let x_max = cmp::min(image.width() - 1, cmp::max(cmp::max(v1.x, v2.x), v3.x));

    let y_min = cmp::min(cmp::min(v1.y, v2.y), v3.y);
    let y_max = cmp::max(cmp::max(v1.y, v2.y), v3.y);

    for x in x_min..x_max {
        for y in y_min..y_max {
            if is_point_in_triangle(v1, v2, v3, Vector2::new(x, y)) {
                image.put_pixel(x as u32, y as u32, color)
            }
        }
    }
}

fn is_point_in_triangle(v1: (Vector2<u32>), v2: (Vector2<u32>), v3: (Vector2<u32>), point: (Vector2<u32>)) -> bool {
    let v1: Vector2<i32> = v1.cast().unwrap();
    let v2: Vector2<i32> = v2.cast().unwrap();
    let v3: Vector2<i32> = v3.cast().unwrap();
    let point: Vector2<i32> = point.cast().unwrap();

    let vec1 = Vector3::new(v3.x - v1.x, v2.x - v1.x, v1.x - point.x);
    let vec2 = Vector3::new(v3.y - v1.y, v2.y - v1.y, v1.y - point.y);
    let u = vec1.cross(vec2);

    if u.z.abs() < 1 {
        return false;
    }

    let barycentric = (1.0 - (u.x + u.y) as f64 / u.z as f64, u.y as f64 / u.z as f64, u.x as f64 / u.z as f64);
    if barycentric.0 < 0.0 || barycentric.1 < 0.0 || barycentric.2 < 0.0 {
        false
    } else {
        true
    }
}
