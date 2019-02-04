use crate::model::Model;

pub fn render_model(model: &Model) {
    let mut image_buffer = image::RgbImage::new(self.x_size as u32, self.y_size as u32);
}

// void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color) {
//     bool steep = false;
//     if (std::abs(x0-x1)<std::abs(y0-y1)) {
//         std::swap(x0, y0);
//         std::swap(x1, y1);
//         steep = true;
//     }
//     if (x0>x1) {
//         std::swap(x0, x1);
//         std::swap(y0, y1);
//     }
//     int dx = x1-x0;
//     int dy = y1-y0;
//     int derror2 = std::abs(dy)*2;
//     int error2 = 0;
//     int y = y0;
//     for (int x=x0; x<=x1; x++) {
//         if (steep) {
//             image.set(y, x, color);
//         } else {
//             image.set(x, y, color);
//         }
//         error2 += derror2;
//         if (error2 > dx) {
//             y += (y1>y0?1:-1);
//             error2 -= dx*2;
//         }
//     }
// }

fn line((x1, y1): (i64, i64), (x2, y2): (i64, i64), image: &image::RgbImage, color: (u32, u32, u32)) {
    let steep: bool;
    if (x1 - x2).abs() < (y1 - y2).abs() {
        let (x1, y1) = (y1, x1);
        let (x2, y2) = (y2, x2);
        steep = true;
    } else {
        steep = false
    }

    if (x1 > x2) {
        let (x1, x2) = (x2, x1);
        let (y1, y2) = (y2, y1);
    }

    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y1;

    for x in x1..x2+1 {
        if steep {
        }
    }
}
