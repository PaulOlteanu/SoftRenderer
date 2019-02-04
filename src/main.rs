mod model;
mod picture;

use crate::model::Model;

fn main() {
    const MODEL_FILE: &str = "test_model.obj";

    let model = Model::new(MODEL_FILE);
    picture::render_model((800, 800), "asdf", &model);
}
