use cgmath::{InnerSpace, Matrix2, Vector2, Vector3};
use image::Pixel;

use crate::model::Model;

pub fn render_model((resolution_x, resolution_y): (u32, u32), file_path: &str, model: &Model) {
    let mut image_buffer = image::RgbImage::new(resolution_x, resolution_y);

    let verts = model.verts();

    // -1.0 / 0.0 is -infinity
    let mut z_buffer: Vec<Vec<f64>> =
        vec![vec![-1.0 / 0.0; resolution_y as usize]; resolution_x as usize];

    let light_direction = Vector3::new(0.0, 0.0, -1.0);

    for face in model.faces() {
        // Triangles are defined counterclockwise to be forward facing
        // The sides chosen and the order of the cross product are important here because of this
        // Here the norm is actually backward-facing,
        // which allows the intensity to just be the dot product rather than the absolute value of the dot product
        let norm = (verts[face.vertices.2] - verts[face.vertices.0])
            .cross(verts[face.vertices.1] - verts[face.vertices.0])
            .normalize();
        let intensity = norm.dot(light_direction);

        if intensity > 0.0 {
            triangle(
                verts[face.vertices.0],
                verts[face.vertices.1],
                verts[face.vertices.2],
                &mut z_buffer,
                &mut image_buffer,
                intensity,
                &model,
            );
        }
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

    for x in x1..=x2 {
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
    v1: Vector3<f64>,
    v2: Vector3<f64>,
    v3: Vector3<f64>,
    z_buf: &mut Vec<Vec<f64>>,
    image: &mut image::RgbImage,
    intensity: f64,
    model: &Model,
) {
    let coordinate_transform_matrix = Matrix2::new(
        f64::from(image.width()) / 2.0,
        0.0,
        0.0,
        f64::from(image.height()) / 2.0,
    );

    let increment = Vector3::new(1.0, 1.0, 1.0);

    let v1_screen: Vector2<u32> = (coordinate_transform_matrix * (v1 + increment).xy())
        .cast()
        .unwrap();

    let v2_screen: Vector2<u32> = (coordinate_transform_matrix * (v2 + increment).xy())
        .cast()
        .unwrap();

    let v3_screen: Vector2<u32> = (coordinate_transform_matrix * (v3 + increment).xy())
        .cast()
        .unwrap();

    let x_min = v1_screen.x.min(v2_screen.x).min(v3_screen.x).max(0);
    let x_max = v1_screen
        .x
        .max(v2_screen.x)
        .max(v3_screen.x)
        .min(image.width() - 1);

    let y_min = v1_screen.y.min(v2_screen.y).min(v3_screen.y).max(0);
    let y_max = v1_screen
        .y
        .max(v2_screen.y)
        .max(v3_screen.y)
        .min(image.height() - 1);

    for x in x_min..x_max {
        for y in y_min..y_max {
            let current_point = Vector2::new(x, y);
            if is_point_in_triangle(v1_screen, v2_screen, v3_screen, current_point) {
                let barycentric = cartesian_to_barycentric(v1_screen, v2_screen, v3_screen, current_point);

                // Interpolate z value across vertices
                let z = v1.z * barycentric.x + v2.z * barycentric.y + v3.z * barycentric.z;

                if z_buf[x as usize][y as usize] < z {
                    z_buf[x as usize][y as usize] = z;

                    let color = get_texture_color(x as f64 / image.width() as f64, 1.0 - y as f64 / image.height() as f64, model).map(|x| {
                        (intensity * x as f64) as u8
                    });

                    image.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}

fn is_point_in_triangle(
    v1: Vector2<u32>,
    v2: Vector2<u32>,
    v3: Vector2<u32>,
    point: Vector2<u32>,
) -> bool {
    let barycentric = cartesian_to_barycentric(v1, v2, v3, point);
    !(barycentric.x < 0.0 || barycentric.y < 0.0 || barycentric.z < 0.0)
}

fn cartesian_to_barycentric(
    v1: Vector2<u32>,
    v2: Vector2<u32>,
    v3: Vector2<u32>,
    point: Vector2<u32>,
) -> Vector3<f64> {
    let v1: Vector2<i32> = v1.cast().unwrap();
    let v2: Vector2<i32> = v2.cast().unwrap();
    let v3: Vector2<i32> = v3.cast().unwrap();
    let point: Vector2<i32> = point.cast().unwrap();

    let vec1 = Vector3::new(v3.x - v1.x, v2.x - v1.x, v1.x - point.x);
    let vec2 = Vector3::new(v3.y - v1.y, v2.y - v1.y, v1.y - point.y);
    let u = vec1.cross(vec2);

    if u.z.abs() < 1 {
        // Doesn't matter what this is as long as one value is negative
        return Vector3::new(0.0, 0.0, -1.0);
    }

    Vector3::new(
        1.0 - f64::from(u.x + u.y) / f64::from(u.z),
        f64::from(u.y) / f64::from(u.z),
        f64::from(u.x) / f64::from(u.z),
    )
}

fn get_texture_color(x: f64, y: f64, model: &Model) -> image::Rgb<u8> {
    model.texture.get_pixel((x * model.texture.width() as f64) as u32, (y * model.texture.height() as f64) as u32 - 1).to_rgb()
}
