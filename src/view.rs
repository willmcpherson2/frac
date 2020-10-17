use crate::model::State;
use quicksilver::geom::Rectangle;
use quicksilver::geom::Transform;
use quicksilver::geom::Vector;
use quicksilver::graphics::Color;
use quicksilver::graphics::ResizeHandler;
use quicksilver::input::Event;
use quicksilver::input::Key;
use quicksilver::Graphics;
use quicksilver::Input;
use quicksilver::Result;
use quicksilver::Window;

pub const INITIAL_SIZE: f32 = 768.0;

pub struct View {
    window: Window,
    graphics: Graphics,
    input: Input,
    size: Vector,
}

impl View {
    pub fn new(window: Window, graphics: Graphics, input: Input) -> Self {
        let mut view = View {
            window,
            graphics,
            input,
            size: Vector::ZERO,
        };
        view.graphics.set_resize_handler(ResizeHandler::Stretch);
        view
    }

    pub async fn get_input(&mut self, state: &mut State) {
        while let Some(event) = self.input.next_event().await {
            match event {
                Event::PointerInput(event) if event.is_down() => {
                    let x_scale = self.window.size().x / state.size_f();
                    let y_scale = self.window.size().y / state.size_f();
                    let x = (self.input.mouse().location().x / x_scale) as usize;
                    let y = (self.input.mouse().location().y / y_scale) as usize;
                    *state.get_mut(x, y) = !state.get(x, y);
                }
                Event::KeyboardInput(event) if event.is_down() => match event.key() {
                    Key::Return => state.next_stage(),
                    Key::Space => state.toggle_running(),
                    Key::Right => *state.step_mut() = true,
                    Key::Escape => state.clear(),
                    Key::S => state.next_strategy(),
                    _ => (),
                },
                _ => (),
            }
        }
    }

    pub fn draw(&mut self, state: &mut State) -> Result<()> {
        self.graphics.clear(Color::WHITE);

        let x_scale = self.window.size().x / state.size_f();
        let y_scale = self.window.size().y / state.size_f();
        let size = Vector::new(x_scale, y_scale);
        for x in 0..state.size_i() {
            for y in 0..state.size_i() {
                let pos = Vector::new(x as f32 * x_scale, y as f32 * y_scale);
                let color = if state.get(x, y) {
                    Color::from_rgba(128, 128, 128, 1.0)
                } else {
                    Color::WHITE
                };
                self.graphics.fill_rect(&Rectangle::new(pos, size), color);
            }
        }

        self.graphics.present(&self.window)
    }

    pub fn resize(&mut self) {
        let new_size = self.window.size();

        if new_size != self.size {
            let scale = Vector::new(INITIAL_SIZE / new_size.x, INITIAL_SIZE / new_size.y);
            self.graphics.set_view(Transform::scale(scale));
            self.size = new_size;
        }
    }
}
