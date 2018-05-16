use wasm_rgame::Color;
use wasm_rgame::delegate_prelude::*;

use cgmath::Vector2;
use std::cell::RefCell;
use std::rc::Rc;

use super::{Transform, TextConfig, Text};

pub struct ButtonConfig {
    pub hovered_color: Color,
    pub clicked_color: Color,
    pub color: Color,

    pub render_order: i32,
}

pub struct Button {
    transform: Transform,
    config: ButtonConfig,
    state: Rc<RefCell<ButtonState>>,

    text_config: Option<TextConfig>,
    text_handle: Option<SpawnHandle<()>>,
}

#[derive(Clone)]
pub struct ButtonHandle {
    state: Rc<RefCell<ButtonState>>,
}

#[derive(Clone)]
struct ButtonState {
    pub hovered: bool,
    pub clicked_in: bool,
    pub clicked: bool,
}

impl Button {
    pub fn new(transform: Transform, config: ButtonConfig) -> Button {
        Button {
            transform,
            config,
            state: Rc::new(RefCell::new(
                ButtonState {
                    hovered: false,
                    clicked_in: false,
                    clicked: false,
                }
            )),
            text_config: None,
            text_handle: None,
        }
    }

    pub fn set_text(&mut self, text_config: TextConfig) {
        self.text_config = Some(text_config);
    }
}

impl Delegate for Button {
    fn on_spawn(
        &mut self,
        delegate_spawner: &mut DelegateSpawner,
    ) {
        if let Some(text_config) = self.text_config.take() {
            self.text_handle = Some(delegate_spawner.spawn(
                Text::new(self.transform.clone(), text_config)
            ));
        }
    }

    fn tick(
        &mut self,
        _context: &mut ApplicationContext,
        _key_manager: &KeyManager,
        mouse_state: &MouseState,
        _delegate_spawner: &mut DelegateSpawner,
    )
    {
        let hovered = self.transform.contains(Vector2 {
            x: mouse_state.pos_x,
            y: mouse_state.pos_y,
        });

        let prev_clicked_in = (*self.state.borrow()).clicked_in;
        let clicked = prev_clicked_in && mouse_state.button_up(MouseButton::Left);
        let clicked_in = if prev_clicked_in {
            // stay clicked in if mouse is still in button transform,
            // if clicked then no longer clicked_in
            hovered && !clicked
        } else {
            // become clicked_in if mouse down in button transform
            hovered && mouse_state.button_down(MouseButton::Left)
        };

        self.state.replace(ButtonState { hovered, clicked_in, clicked, });
    }

    fn render(&self, graphics: &mut Graphics) {
        let bottom_left = self.transform.bottom_left();
        let size = self.transform.size;

        let state = self.state.borrow().clone();
        let color = if state.clicked_in || state.clicked {
            self.config.clicked_color
        } else if state.hovered {
            self.config.hovered_color
        } else {
            self.config.color
        };

        graphics.draw_rect(
            bottom_left.x,
            bottom_left.y,
            size.x,
            size.y,
            color,
        );
    }

    fn render_order(&self) -> i32 { self.config.render_order }
}

impl SpawnableDelegate for Button {
    type Handle = ButtonHandle;

    fn handle(&self) -> Self::Handle {
        ButtonHandle { state: self.state.clone(), }
    }
}

impl ButtonHandle {
    pub fn hovered(&self) -> bool {
        (*self.state.borrow()).hovered
    }

    pub fn clicked(&self) -> bool {
        (*self.state.borrow()).clicked
    }
}
