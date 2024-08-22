use bevy::color::palettes::css::{GREEN, ORANGE, PURPLE, WHITE, YELLOW};
use bevy::prelude::*;
use bevy::render::render_resource::Texture;

/// テクスチャと色を持ったスプライトのマテリアル
#[derive(Debug, Clone)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            texture: Default::default(),
        }
    }
}

/// ボードのアセット、リソースとして扱う
#[derive(Debug, Clone, Resource)]
pub struct BoardAssets {
    pub label: String,
    pub board_material: SpriteMaterial,
    pub tile_material: SpriteMaterial,
    pub covered_tile_material: SpriteMaterial,
    pub bomb_counter_font: Handle<Font>,
    pub bomb_counter_colors: Vec<Color>,
    pub flag_material: SpriteMaterial,
    pub bomb_material: SpriteMaterial,
}

impl BoardAssets {
    pub fn default_colors() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::from(GREEN),
            Color::from(YELLOW),
            Color::from(ORANGE),
            Color::from(PURPLE),
        ]
    }
    /// 爆弾カウンターに一致する色を安全に取得する
    pub fn bomb_counter_color(&self, counter: u8) -> Color {
        let counter = counter.saturating_sub(1) as usize;
        match self.bomb_counter_colors.get(counter) {
            Some(c) => *c,
            None => match self.bomb_counter_colors.last() {
                None => { Color::from(WHITE) }
                Some(c) => *c,
            },
        }
    }
}