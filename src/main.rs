mod model;
mod picture;

use crate::model::Model;

fn main() {
    const MODEL_FILE: &str = "test_model.obj";
    const TEXTURE_FILE: &str = "test_model.tga";

    let model = Model::new(MODEL_FILE, TEXTURE_FILE);
    picture::render_model((800, 800), "output", &model);
}
