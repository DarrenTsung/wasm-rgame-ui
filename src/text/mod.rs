use wasm_rgame::Color;
use wasm_rgame::delegate_prelude::*;

use super::{Transform};

/// Because fontSize / 2.0 doesn't aligned the font in the center,
/// we use this magic variable to scale fontSize
const FONT_SIZE_HEIGHT_RATIO: f32 = 0.6;

pub struct TextConfig {
    pub color: Color,
    pub text: String,
    pub font_size: f32,

    pub render_order: i32,
}

pub struct Text {
    transform: Transform,
    config: TextConfig,
}

impl Text {
    pub fn new(transform: Transform, config: TextConfig) -> Text {
        Text {
            transform,
            config,
        }
    }
}

impl Delegate for Text {
    fn tick(
        &mut self,
        _context: &mut ApplicationContext,
        _key_manager: &KeyManager,
        _mouse_manager: &MouseManager,
        _delegate_spawner: &mut DelegateSpawner,
    ) {}

    fn render(&self, graphics: &mut Graphics) {
        let center = self.transform.center();
        graphics.draw_string(
            &self.config.text,
            center.x,
            center.y - (self.config.font_size * FONT_SIZE_HEIGHT_RATIO / 2.0),
            self.config.font_size,
            self.config.color,
        );
    }

    fn render_order(&self) -> i32 { self.config.render_order }
}

impl SpawnableDelegate for Text {
    type Handle = ();

    fn handle(&self) -> Self::Handle {
        ()
    }
}
