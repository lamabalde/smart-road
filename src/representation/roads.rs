use crate::representation::textures::Textures;

use macroquad::prelude::*;

pub fn representation_textured_roads(textures: &Textures) {
    draw_texture(&textures.bg, 0.0, 0.0, WHITE);
}
