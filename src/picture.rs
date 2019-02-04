use crate::model::Model;

pub fn render_model((resolution_x, resolution_y): (u32, u32), file_path: &str, model: &Model) {
    let mut image_buffer = image::RgbImage::new(resolution_x, resolution_y);
    line((1, 1), (5, 2), &mut image_buffer, (255, 255, 255));
    image_buffer.save(format!("{}.png", file_path)).unwrap();
}

fn line(start: (i64, i64), end: (i64, i64), image: &mut image::RgbImage, color: (u8, u8, u8)) {
    let x1 = start.0 - 1;
    let y1 = start.1 - 1;
    let x2 = end.0 - 1;
    let y2 = end.1 - 1;
    let steep: bool;

    // If dy > dx then the line is "steep", idk why swap
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

    // Dunno what these are
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y1;

    for x in x1..x2 + 1 {
        if steep {
            // Unswap x and y
            image.put_pixel(y as u32, image.height() - x as u32 - 1, image::Rgb([color.0, color.1, color.2]));
        } else {
            image.put_pixel(x as u32, image.height() - y as u32 - 1, image::Rgb([color.0, color.1, color.2]));
        }

        error2 += derror2;

        if error2 > dx {
            y += if y2 > y1 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}
