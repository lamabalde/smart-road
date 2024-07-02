use crate::circulation::Model;
use crate::{
    config::SECTOR_WIDTH,
    circulation::{car::Car, Moving},
};
use macroquad::prelude::*;
pub fn representation_car(car: &Car, textures: &[Texture2D]) {
    let texture = match car.model {
        Model::Standard => &textures[0],
        Model::Sport => &textures[1],
        Model::TaxiVert => &textures[2],
        Model::TaxiOrange => &textures[3],
        Model::TaxiNoire => &textures[4],
        Model::TaxiBleu => &textures[5],
        Model::TaxiRouge => &textures[6],
    };
    // Déterminer quel sprite utiliser en fonction de la direction de la voiture.
    let rotation: f32 = match car.moving {
        Moving::Up => 0.0,
        Moving::Left => -90.0,
        Moving::Down => -180.0,
        Moving::Right => -270.0,
    };
    let src_rect = Rect::new(0.0, 0.0, SECTOR_WIDTH, SECTOR_WIDTH);
    // réduire à l'échelle de 80%
    let scaled_size = SECTOR_WIDTH * 0.9;
    // Calculer la position pour centrer la voiture dans le secteur
    let center_x = car.x + (SECTOR_WIDTH - scaled_size) / 2.0;
    let center_y = car.y + (SECTOR_WIDTH - scaled_size) / 2.0;

    draw_texture_ex(
        texture,
        center_x,
        center_y,
        WHITE,
        DrawTextureParams {
            source: Some(src_rect),
            rotation: rotation.to_radians(),
            dest_size: Some(Vec2::new(scaled_size, scaled_size)), // Définir à 80% de la taille du secteur
            ..Default::default()
        },
    );
}
