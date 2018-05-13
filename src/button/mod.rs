use wasm_rgame::{MouseButton, MouseButtonState};
use wasm_rgame::Color;
use wasm_rgame::delegate_prelude::*;

use cgmath::Vector2;
use std::cell::RefCell;
use std::rc::Rc;

use super::{Transform};

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
    mouse_position: Vector2<f64>,
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
            mouse_position: Vector2 { x: 0.0, y: 0.0, },
        }
    }
}

impl Delegate for Button {
    fn tick(
        &mut self,
        _context: &mut ApplicationContext,
        _key_manager: &KeyManager,
        mouse_manager: &MouseManager,
        _delegate_spawner: &mut DelegateSpawner,
    )
    {
        let hovered = self.transform.contains(Vector2 {
            x: mouse_manager.pos_x.into(),
            y: mouse_manager.pos_y.into(),
        });

        self.mouse_position = Vector2 {
            x: mouse_manager.pos_x.into(),
            y: mouse_manager.pos_y.into(),
        };

        let prev_clicked_in = (*self.state.borrow()).clicked_in;
        let clicked = prev_clicked_in && mouse_manager.button_state(MouseButton::Left) == MouseButtonState::Up;
        let clicked_in = if prev_clicked_in {
            // stay clicked in if mouse is still in button transform,
            // if clicked then no longer clicked_in
            hovered && !clicked
        } else {
            // become clicked_in if mouse down in button transform
            hovered && mouse_manager.button_state(MouseButton::Left) == MouseButtonState::Down
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
            bottom_left.x as i32,
            bottom_left.y as i32,
            size.x as u16,
            size.y as u16,
            color,
        );

        graphics.draw_rect(
            self.mouse_position.x as i32,
            self.mouse_position.y as i32,
            10,
            10,
            [0, 0, 0, 255],
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
