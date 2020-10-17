mod model;
mod view;

use crate::model::State;
use crate::view::View;
use quicksilver::geom::Vector;
#[allow(unused_imports)]
use quicksilver::log::info;
use quicksilver::log::Level;
use quicksilver::run;
use quicksilver::Graphics;
use quicksilver::Input;
use quicksilver::Result;
use quicksilver::Settings;
use quicksilver::Window;

fn main() {
    run(
        Settings {
            title: "frac",
            log_level: Level::Info,
            resizable: true,
            size: Vector::new(view::INITIAL_SIZE, view::INITIAL_SIZE),
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, graphics: Graphics, input: Input) -> Result<()> {
    let mut view = View::new(window, graphics, input);
    let mut state = State::new();

    loop {
        view.get_input(&mut state).await;
        state.update();
        view.resize();
        view.draw(&mut state)?;
    }
}
